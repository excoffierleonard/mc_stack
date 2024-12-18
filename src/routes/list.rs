use actix_web::{get, Error, HttpResponse, ResponseError};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use tokio::process::Command;
use walkdir::WalkDir;

#[derive(Debug)]
enum ListStackError {
    FileSystemError(String),
    DockerError(String),
}

impl fmt::Display for ListStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileSystemError(msg) | Self::DockerError(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl ResponseError for ListStackError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ListStackError::FileSystemError(_) | ListStackError::DockerError(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
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

// Maybe don't automatically create on dir on dir not found
async fn get_stacks_directory() -> Result<PathBuf, ListStackError> {
    let current_exe = std::env::current_exe().map_err(|e| {
        ListStackError::FileSystemError(format!("Failed to get current path: {}", e))
    })?;

    let stacks_dir = current_exe
        .parent()
        .ok_or_else(|| {
            ListStackError::FileSystemError("Failed to find executable directory".to_string())
        })?
        .join("stacks");

    if !stacks_dir.exists() {
        fs::create_dir_all(&stacks_dir).map_err(|e| {
            ListStackError::FileSystemError(format!("Failed to create stacks directory: {}", e))
        })?;
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
            "Failed to get container information".to_string(),
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

        let port = ports
            .split(',')
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
        let entry = entry.map_err(|e| {
            ListStackError::FileSystemError(format!("Failed to read directory entry: {}", e))
        })?;

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

    if stacks.is_empty() {
        return Ok(HttpResponse::NoContent().finish());
    }

    // TODO: Hardcoded WAN IP for now or performance reasons,
    // must find way to get it from the running dockers themselves to save some loading time,
    // currently it takes ~200ms to get the WAN IP by fetching it from the web,
    // objective is to get it under 20ms
    let wan_ip = "24.48.49.227".to_string();

    // Get running containers
    let containers = get_running_containers().await?;

    // Build stacks status
    let stack_statuses: Vec<Value> = stacks
        .iter()
        .map(|stack_id| {
            let sftp_name = format!("sftp_server_{}", stack_id);
            let minecraft_name = format!("minecraft_server_{}", stack_id);

            let sftp_status = containers
                .get(&sftp_name)
                .cloned()
                .unwrap_or(ServiceStatus {
                    status: "stopped".to_string(),
                    port: None,
                });

            let minecraft_status =
                containers
                    .get(&minecraft_name)
                    .cloned()
                    .unwrap_or(ServiceStatus {
                        status: "stopped".to_string(),
                        port: None,
                    });

            json!({
                "stack_id": stack_id,
                "wan_ip": wan_ip,
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
        })
        .collect();

    Ok(HttpResponse::Ok().json(stack_statuses))
}

#[get("/stacks")]
pub async fn list_stacks() -> Result<HttpResponse, Error> {
    list_stacks_impl().await
}
