use actix_web::{post, web, Error, HttpResponse, ResponseError};
use serde_json::json;
use std::path::PathBuf;
use std::fmt;
use tokio::process::Command;

#[derive(Debug)]
enum StopStackError {
    StackNotFound(String),
    DockerError(String),
}

impl fmt::Display for StopStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackNotFound(msg) | Self::DockerError(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl ResponseError for StopStackError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            StopStackError::StackNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            StopStackError::DockerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(json!({ "message": self.to_string() }))
    }
}

async fn get_compose_file_path(stack_id: &str) -> Result<PathBuf, StopStackError> {
    let current_exe = std::env::current_exe()
        .map_err(|e| StopStackError::DockerError(format!("Failed to get current path: {}", e)))?;
    
    let stack_dir = current_exe
        .parent() // bin directory
        .ok_or_else(|| StopStackError::DockerError("Failed to find executable directory".to_string()))?
        .join("stacks")
        .join(format!("stack_{}", stack_id))
        .join("compose.yaml");

    if !stack_dir.exists() {
        return Err(StopStackError::StackNotFound(
            format!("Stack {} does not exist", stack_id)
        ));
    }

    Ok(stack_dir)
}

async fn stop_stack_impl(stack_id: String) -> Result<HttpResponse, Error> {
    // Get compose file path
    let compose_file = get_compose_file_path(&stack_id).await?;

    // Run docker compose down
    let output = Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_file.to_str().unwrap(),
            "down"
        ])
        .output()
        .await
        .map_err(|e| StopStackError::DockerError(
            format!("Failed to execute docker compose: {}", e)
        ))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(StopStackError::DockerError(
            format!("Failed to stop stack {}: {}", stack_id, error_msg)
        ))?;
    }

    Ok(HttpResponse::Ok().json(json!({
        "message": format!("Stack {} has been successfully stopped", stack_id)
    })))
}

#[post("/{stack_id}")]
pub async fn stop_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    stop_stack_impl(stack_id.into_inner()).await
}