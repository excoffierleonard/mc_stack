use actix_web::{delete, web, Error, HttpResponse, ResponseError};
use serde_json::json;
use std::path::PathBuf;
use std::fmt;
use tokio::process::Command;
use tokio::fs;

#[derive(Debug)]
enum DeleteStackError {
    StackNotFound(String),
    DockerError(String),
    FileSystemError(String),
}

impl fmt::Display for DeleteStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackNotFound(msg) | Self::DockerError(msg) | Self::FileSystemError(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl ResponseError for DeleteStackError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            DeleteStackError::StackNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            DeleteStackError::DockerError(_) | DeleteStackError::FileSystemError(_) => {
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

async fn get_compose_file_path(stack_id: &str) -> Result<PathBuf, DeleteStackError> {
    let current_exe = std::env::current_exe()
        .map_err(|e| DeleteStackError::FileSystemError(format!("Failed to get current path: {}", e)))?;
    
    let stack_dir = current_exe
        .parent() // bin directory
        .ok_or_else(|| DeleteStackError::FileSystemError("Failed to find executable directory".to_string()))?
        .join("stacks")
        .join(format!("stack_{}", stack_id))
        .join("compose.yaml");

    if !stack_dir.exists() {
        return Err(DeleteStackError::StackNotFound(
            format!("Stack {} does not exist", stack_id)
        ));
    }

    Ok(stack_dir)
}

async fn delete_stack_impl(stack_id: String) -> Result<HttpResponse, Error> {
    // Get compose file path and stack directory
    let compose_file = get_compose_file_path(&stack_id).await?;
    let stack_dir = compose_file.parent()
        .ok_or_else(|| DeleteStackError::FileSystemError("Failed to get stack directory".to_string()))?;

    // Step 1: Stop the stack using docker compose down
    let output = Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_file.to_str().unwrap(),
            "down"
        ])
        .output()
        .await
        .map_err(|e| DeleteStackError::DockerError(
            format!("Failed to execute docker compose down: {}", e)
        ))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(DeleteStackError::DockerError(
            format!("Failed to stop stack {}: {}", stack_id, error_msg)
        ))?;
    }

    // Step 2: Remove the Docker volume
    let volume_name = format!("minecraft_server_{}", stack_id);
    let output = Command::new("docker")
        .args([
            "volume",
            "rm",
            &volume_name
        ])
        .output()
        .await
        .map_err(|e| DeleteStackError::DockerError(
            format!("Failed to remove docker volume: {}", e)
        ))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(DeleteStackError::DockerError(
            format!("Failed to remove minecraft server volume: {}", error_msg)
        ))?;
    }

    // Step 3: Remove the stack directory
    fs::remove_dir_all(stack_dir).await
        .map_err(|e| DeleteStackError::FileSystemError(
            format!("Failed to remove stack directory: {}", e)
        ))?;

    Ok(HttpResponse::Ok().json(json!({
        "message": format!("Stack {} has been successfully deleted", stack_id)
    })))
}

#[delete("/{stack_id}")]
pub async fn delete_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    delete_stack_impl(stack_id.into_inner()).await
}