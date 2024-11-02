use actix_web::{put, web, Error, HttpResponse};
use serde_json::json;
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Debug)]
enum StartStackError {
    StackNotFound(String),
    DockerError(String),
}

impl StartStackError {
    fn to_http_response(&self) -> Result<HttpResponse, Error> {
        let (status, message) = match self {
            StartStackError::StackNotFound(msg) => (
                actix_web::http::StatusCode::NOT_FOUND,
                msg.clone(),
            ),
            StartStackError::DockerError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
            ),
        };

        Ok(HttpResponse::build(status)
            .content_type("application/json")
            .json(json!({ "message": message })))
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
    let compose_file = match get_compose_file_path(&stack_id).await {
        Ok(path) => path,
        Err(e) => {
            log::error!("Failed to get compose file path: {:?}", e);
            return e.to_http_response();
        }
    };

    let output = match Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_file.to_str().unwrap(),
            "up",
            "-d"
        ])
        .output()
        .await
    {
        Ok(output) => output,
        Err(e) => {
            let error = StartStackError::DockerError(
                format!("Failed to execute docker compose: {}", e)
            );
            return error.to_http_response();
        }
    };

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        let error = StartStackError::DockerError(
            format!("Failed to start stack {}: {}", stack_id, error_msg)
        );
        return error.to_http_response();
    }

    Ok(HttpResponse::Ok().json(json!({
        "message": format!("Stack {} has been successfully started", stack_id)
    })))
}

#[put("/{stack_id}")]
pub async fn start_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    start_stack_impl(stack_id.into_inner()).await
}