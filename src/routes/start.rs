use actix_web::{put, web, Error, HttpResponse, ResponseError};
use serde_json::json;
use std::path::PathBuf;
use std::fmt;
use tokio::process::Command;

#[derive(Debug)]
enum StartStackError {
    StackNotFound(String),
    DockerError(String),
}

impl fmt::Display for StartStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackNotFound(msg) | Self::DockerError(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl ResponseError for StartStackError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            StartStackError::StackNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            StartStackError::DockerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(json!({ "message": self.to_string() }))
    }
}

async fn get_compose_file_path(stack_id: &str) -> Result<PathBuf, StartStackError> {
    let current_exe = std::env::current_exe()
        .map_err(|e| StartStackError::DockerError(format!("Failed to get current path: {}", e)))?;
    
    let stack_dir = current_exe
        .parent() // bin directory
        .ok_or_else(|| StartStackError::DockerError("Failed to find executable directory".to_string()))?
        .join("stacks")
        .join(format!("stack_{}", stack_id))
        .join("compose.yaml");

    if !stack_dir.exists() {
        return Err(StartStackError::StackNotFound(
            format!("Stack {} does not exist", stack_id)
        ));
    }

    Ok(stack_dir)
}

async fn start_stack_impl(stack_id: String) -> Result<HttpResponse, Error> {
    // Get compose file path
    let compose_file = get_compose_file_path(&stack_id).await?;

    // Run docker compose up
    let output = Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_file.to_str().unwrap(),
            "up",
            "-d"
        ])
        .output()
        .await
        .map_err(|e| StartStackError::DockerError(
            format!("Failed to execute docker compose: {}", e)
        ))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(StartStackError::DockerError(
            format!("Failed to start stack {}: {}", stack_id, error_msg)
        ))?;
    }

    Ok(HttpResponse::Ok().json(json!({
        "message": format!("Stack {} has been successfully started", stack_id)
    })))
}

#[put("/{stack_id}")]
pub async fn start_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    start_stack_impl(stack_id.into_inner()).await
}