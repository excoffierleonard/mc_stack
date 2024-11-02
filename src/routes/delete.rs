// src/routes/delete.rs
use actix_web::{delete, web, Error, HttpResponse};
use serde_json::json;
use std::path::PathBuf;
use tokio::process::Command;
use tokio::fs;

#[derive(Debug)]
enum DeleteStackError {
    StackNotFound(String),
    DockerError(String),
    FileSystemError(String),
}

impl DeleteStackError {
    fn to_http_response(&self) -> Result<HttpResponse, Error> {
        let (status, message) = match self {
            DeleteStackError::StackNotFound(msg) => (
                actix_web::http::StatusCode::NOT_FOUND,
                msg.clone(),
            ),
            DeleteStackError::DockerError(msg) | DeleteStackError::FileSystemError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
            ),
        };

        Ok(HttpResponse::build(status)
            .content_type("application/json")
            .json(json!({ "message": message })))
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
    let compose_file = match get_compose_file_path(&stack_id).await {
        Ok(path) => path,
        Err(e) => {
            log::error!("Failed to get compose file path: {:?}", e);
            return e.to_http_response();
        }
    };

    let stack_dir = match compose_file.parent() {
        Some(dir) => dir,
        None => {
            let error = DeleteStackError::FileSystemError("Failed to get stack directory".to_string());
            return error.to_http_response();
        }
    };

    // Step 1: Stop the stack using docker compose down
    let output = match Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_file.to_str().unwrap(),
            "down"
        ])
        .output()
        .await
    {
        Ok(output) => output,
        Err(e) => {
            let error = DeleteStackError::DockerError(
                format!("Failed to execute docker compose down: {}", e)
            );
            return error.to_http_response();
        }
    };

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        let error = DeleteStackError::DockerError(
            format!("Failed to stop stack {}: {}", stack_id, error_msg)
        );
        return error.to_http_response();
    }

    // Step 2: Remove the Docker volume
    let volume_name = format!("minecraft_server_{}", stack_id);
    let output = match Command::new("docker")
        .args([
            "volume",
            "rm",
            &volume_name
        ])
        .output()
        .await
    {
        Ok(output) => output,
        Err(e) => {
            let error = DeleteStackError::DockerError(
                format!("Failed to remove docker volume: {}", e)
            );
            return error.to_http_response();
        }
    };

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        let error = DeleteStackError::DockerError(
            format!("Failed to remove minecraft server volume: {}", error_msg)
        );
        return error.to_http_response();
    }

    // Step 3: Remove the stack directory
    match fs::remove_dir_all(stack_dir).await {
        Ok(_) => (),
        Err(e) => {
            let error = DeleteStackError::FileSystemError(
                format!("Failed to remove stack directory: {}", e)
            );
            return error.to_http_response();
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "message": format!("Stack {} has been successfully deleted", stack_id)
    })))
}

#[delete("/{stack_id}")]
pub async fn delete_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    delete_stack_impl(stack_id.into_inner()).await
}