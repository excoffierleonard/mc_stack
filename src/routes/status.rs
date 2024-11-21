use actix_web::{patch, web, Error, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusUpdate {
    status: String,
}

#[derive(Debug, Clone)]
pub enum StackStatus {
    Running,
    Stopped,
}

#[derive(Debug)]
pub enum StackError {
    // Made public
    StackNotFound(String),
    DockerError(String),
    InvalidStatus(String),
}

impl TryFrom<String> for StackStatus {
    type Error = StackError;

    fn try_from(status: String) -> Result<Self, Self::Error> {
        match status.to_lowercase().as_str() {
            "running" => Ok(StackStatus::Running),
            "stopped" => Ok(StackStatus::Stopped),
            _ => Err(StackError::InvalidStatus(format!(
                "Invalid status value: '{}'. Must be 'running' or 'stopped'",
                status
            ))),
        }
    }
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackNotFound(msg) | Self::DockerError(msg) | Self::InvalidStatus(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl ResponseError for StackError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            StackError::StackNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            StackError::DockerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            StackError::InvalidStatus(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(json!({ "message": self.to_string() }))
    }
}

async fn get_compose_file_path(stack_id: &str) -> Result<PathBuf, StackError> {
    let current_exe = std::env::current_exe()
        .map_err(|e| StackError::DockerError(format!("Failed to get current path: {}", e)))?;

    let stack_dir = current_exe
        .parent()
        .ok_or_else(|| StackError::DockerError("Failed to find executable directory".to_string()))?
        .join("stacks")
        .join(format!("stack_{}", stack_id))
        .join("compose.yaml");

    if !stack_dir.exists() {
        return Err(StackError::StackNotFound(format!(
            "Stack {} does not exist",
            stack_id
        )));
    }

    Ok(stack_dir)
}

async fn update_stack_status_impl(
    stack_id: String,
    status_update: StatusUpdate,
) -> Result<HttpResponse, Error> {
    // Convert and validate status
    let status = StackStatus::try_from(status_update.status)?;

    let compose_file = get_compose_file_path(&stack_id).await?;

    let docker_command = match status {
        StackStatus::Running => vec!["up", "-d"],
        StackStatus::Stopped => vec!["down"],
    };

    let mut cmd = Command::new("docker");
    cmd.args(["compose", "-f", compose_file.to_str().unwrap()]);
    cmd.args(&docker_command);

    let output = cmd
        .output()
        .await
        .map_err(|e| StackError::DockerError(format!("Failed to execute docker compose: {}", e)))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(StackError::DockerError(format!(
            "Failed to update stack {} status: {}",
            stack_id, error_msg
        )))?;
    }

    Ok(HttpResponse::NoContent().finish())
}

#[patch("/stacks/{stack_id}/status")]
pub async fn update_stack_status(
    stack_id: web::Path<String>,
    status: web::Json<StatusUpdate>,
) -> Result<HttpResponse, Error> {
    update_stack_status_impl(stack_id.into_inner(), status.into_inner()).await
}
