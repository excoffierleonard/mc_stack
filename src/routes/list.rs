use actix_web::{get, Error, HttpResponse, ResponseError};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::fmt;
use tokio::process::Command;
use walkdir::WalkDir;
use std::collections::HashMap;

#[derive(Debug)]
enum ListStackError {
    DirectoryError(String),
    DockerError(String),
}

impl fmt::Display for ListStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirectoryError(msg) | Self::DockerError(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl ResponseError for ListStackError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ListStackError::DirectoryError(_) => actix_web::http::StatusCode::NOT_FOUND,
            ListStackError::DockerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(json!({ "message": self.to_string() }))
    }
}

#[derive(Debug, Clone)]
struct ServiceStatus {
    status: String,
    port: Option<String>,
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
    let stacks_dir = get_stacks_directory().await?;

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
        let entry = entry.map_err(|e| ListStackError::DirectoryError(
            format!("Failed to read directory entry: {}", e)
        ))?;
        
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

    // TODO: Hardcoded WAN IP for now or performance reasons, 
    // must find way to get it from the running dockers themeslesves to say some loading time,
    // currently it takes ~200ms to get the WAN IP by fetching it from the web,
    // objective is to get it under 20ms
    let wan_ip = "24.48.49.227".to_string();

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
    let containers = get_running_containers().await?;

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