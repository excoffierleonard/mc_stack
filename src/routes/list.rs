use actix_web::{get, Error, HttpResponse};
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::process::Command;
use walkdir::WalkDir;
use reqwest;
use std::collections::HashMap;

#[derive(Debug)]
enum ListStackError {
    DirectoryError(String),
    DockerError(String),
    NetworkError(String),
}

impl ListStackError {
    fn to_http_response(&self) -> Result<HttpResponse, Error> {
        let (status, message) = match self {
            ListStackError::DirectoryError(msg) => (
                actix_web::http::StatusCode::NOT_FOUND,
                msg.clone(),
            ),
            ListStackError::DockerError(msg) | ListStackError::NetworkError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
            ),
        };

        Ok(HttpResponse::build(status)
            .content_type("application/json")
            .json(json!({ "message": message })))
    }
}

#[derive(Debug, Clone)]
struct ServiceStatus {
    status: String,
    port: Option<String>,
}

async fn get_wan_ip() -> Result<String, ListStackError> {
    let ip = reqwest::get("http://ipinfo.io/ip")
        .await
        .map_err(|e| ListStackError::NetworkError(format!("Failed to get WAN IP: {}", e)))?
        .text()
        .await
        .map_err(|e| ListStackError::NetworkError(format!("Failed to read WAN IP response: {}", e)))?
        .trim()
        .to_string();
    
    Ok(ip)
}

async fn get_stacks_directory() -> Result<PathBuf, ListStackError> {
    let current_exe = std::env::current_exe()
        .map_err(|e| ListStackError::DirectoryError(format!("Failed to get current path: {}", e)))?;
    
    let stacks_dir = current_exe
        .parent()
        .ok_or_else(|| ListStackError::DirectoryError("Failed to find executable directory".to_string()))?
        .join("stacks");

    if !stacks_dir.exists() {
        return Err(ListStackError::DirectoryError(
            format!("Stacks directory {:?} does not exist", stacks_dir)
        ));
    }

    Ok(stacks_dir)
}

async fn get_running_containers() -> Result<HashMap<String, ServiceStatus>, ListStackError> {
    let output = Command::new("docker")
        .args(["ps", "--format", "{{.Names}}|{{.Ports}}"])
        .output()
        .await
        .map_err(|e| ListStackError::DockerError(format!("Failed to execute docker ps: {}", e)))?;

    if !output.status.success() {
        return Err(ListStackError::DockerError(
            "Failed to get container information".to_string()
        ));
    }

    let containers_str = String::from_utf8_lossy(&output.stdout);
    let mut container_map = HashMap::new();

    for line in containers_str.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            continue;
        }

        let name = parts[0];
        let ports = parts[1];

        // Extract port from the ports string (looking for 0.0.0.0:PORT pattern)
        let port = ports.split(',')
            .filter_map(|p| p.trim().split(':').nth(1))
            .filter_map(|p| p.split('-').next())
            .next()
            .map(|p| p.to_string());

        container_map.insert(
            name.to_string(),
            ServiceStatus {
                status: "running".to_string(),
                port,
            },
        );
    }

    Ok(container_map)
}

async fn list_stacks_impl() -> Result<HttpResponse, Error> {
    // Get stacks directory
    let stacks_dir = match get_stacks_directory().await {
        Ok(dir) => dir,
        Err(e) => return e.to_http_response(),
    };

    // Get all compose files
    let mut stacks = Vec::new();
    for entry in WalkDir::new(&stacks_dir)
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|s| s == "compose.yaml")
                .unwrap_or(false)
        })
    {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                let error = ListStackError::DirectoryError(
                    format!("Failed to read directory entry: {}", e)
                );
                return error.to_http_response();
            }
        };
        
        let stack_dir = entry.path().parent().unwrap();
        let stack_id = stack_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('_')
            .nth(1)
            .unwrap()
            .to_string();

        stacks.push(stack_id);
    }

    // Get WAN IP first
    let wan_ip = get_wan_ip().await.unwrap_or_default();

    if stacks.is_empty() {
        return Ok(HttpResponse::Ok().json(json!({
            "message": "Stack status retrieved successfully",
            "data": {
                "wan_ip": wan_ip,
                "stacks": []
            }
        })));
    }

    // Get running containers
    let containers = match get_running_containers().await {
        Ok(c) => c,
        Err(e) => return e.to_http_response(),
    };

    // Build stacks status
    let stack_statuses: Vec<Value> = stacks.iter().map(|stack_id| {
        let sftp_name = format!("sftp_server_{}", stack_id);
        let minecraft_name = format!("minecraft_server_{}", stack_id);

        let sftp_status = containers.get(&sftp_name).cloned().unwrap_or(ServiceStatus {
            status: "stopped".to_string(),
            port: None,
        });

        let minecraft_status = containers.get(&minecraft_name).cloned().unwrap_or(ServiceStatus {
            status: "stopped".to_string(),
            port: None,
        });

        json!({
            "stack_id": stack_id,
            "services": {
                "sftp_server": {
                    "status": sftp_status.status,
                    "port": sftp_status.port
                },
                "minecraft_server": {
                    "status": minecraft_status.status,
                    "port": minecraft_status.port
                }
            }
        })
    }).collect();

    Ok(HttpResponse::Ok().json(json!({
        "message": "Stack status retrieved successfully",
        "data": {
            "wan_ip": wan_ip,
            "stacks": stack_statuses
        }
    })))
}

#[get("/list")]
pub async fn list_stacks() -> Result<HttpResponse, Error> {
    list_stacks_impl().await
}