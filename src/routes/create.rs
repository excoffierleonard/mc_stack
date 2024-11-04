use actix_web::{post, Error, HttpResponse, ResponseError};
use serde_json::json;
use std::path::PathBuf;
use std::fmt;
use tokio::process::Command;
use regex::Regex;
use num_cpus;
use std::fs;

const INCREMENT: i32 = 3;
const ENV_TEMPLATE: &str = include_str!("../../template/.env");
const COMPOSE_TEMPLATE: &str = include_str!("../../template/compose.yaml");

#[derive(Debug)]
enum CreateStackError {
    ValidationError(String),
    FileSystemError(String),
    DockerError(String),
}

impl fmt::Display for CreateStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidationError(msg) | Self::FileSystemError(msg) | Self::DockerError(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl ResponseError for CreateStackError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            CreateStackError::ValidationError(_) => actix_web::http::StatusCode::FORBIDDEN,
            CreateStackError::FileSystemError(_) | CreateStackError::DockerError(_) => {
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

struct EnvConfig {
    server_port: i32,
    rcon_port: i32,
    sftp_port: i32,
}

fn parse_env_template() -> Result<EnvConfig, CreateStackError> {
    let get_port = |var_name: &str| -> Result<i32, CreateStackError> {
        let re = Regex::new(&format!(r"^{}=(\d+)", var_name)).unwrap();
        ENV_TEMPLATE
            .lines()
            .find_map(|line| re.captures(line))
            .and_then(|cap| cap[1].parse().ok())
            .ok_or_else(|| CreateStackError::ValidationError(format!("{} not found in env template", var_name)))
    };

    Ok(EnvConfig {
        server_port: get_port("SERVER_PORT")?,
        rcon_port: get_port("RCON_PORT")?,
        sftp_port: get_port("SFTP_SERVER_PORT")?,
    })
}

async fn get_stacks_directory() -> Result<PathBuf, CreateStackError> {
    let current_exe = std::env::current_exe()
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to get current path: {}", e)))?;
    
    let stacks_dir = current_exe
        .parent()
        .ok_or_else(|| CreateStackError::FileSystemError("Failed to find executable directory".to_string()))?
        .join("stacks");

    if !stacks_dir.exists() {
        fs::create_dir_all(&stacks_dir)
            .map_err(|e| CreateStackError::FileSystemError(format!("Failed to create stacks directory: {}", e)))?;
    }

    Ok(stacks_dir)
}

async fn create_stack_impl() -> Result<HttpResponse, Error> {
    let stacks_dir = get_stacks_directory().await?;

    // Check maximum stacks limit
    let max_stacks = num_cpus::get();
    let stack_count = fs::read_dir(&stacks_dir)
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to read stacks directory: {}", e)))?
        .filter(|entry| {
            entry.as_ref()
                .map(|e| e.file_name().to_string_lossy().starts_with("stack_"))
                .unwrap_or(false)
        })
        .count();

    if stack_count >= max_stacks {
        return Err(CreateStackError::ValidationError(
            format!("Maximum number of stacks ({}) reached", max_stacks)
        ))?;
    }

    // Find highest existing stack number
    let mut highest_number = 0;
    for entry in fs::read_dir(&stacks_dir)
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to read stacks directory: {}", e)))? {
        let entry = entry.map_err(|e| CreateStackError::FileSystemError(format!("Failed to read directory entry: {}", e)))?;
        if let Some(num_str) = entry.file_name().to_string_lossy().strip_prefix("stack_") {
            if let Ok(num) = num_str.parse::<i32>() {
                highest_number = highest_number.max(num);
            }
        }
    }

    let new_stack_id = highest_number + 1;
    let new_stack_dir = stacks_dir.join(format!("stack_{}", new_stack_id));

    let env_config = parse_env_template()?;
    
    let new_server_port = env_config.server_port + new_stack_id * INCREMENT;
    let new_rcon_port = env_config.rcon_port + new_stack_id * INCREMENT;
    let new_sftp_port = env_config.sftp_port + new_stack_id * INCREMENT;

    // Create new stack directory
    fs::create_dir_all(&new_stack_dir)
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to create stack directory: {}", e)))?;

    // Create env file with updated values
    let new_content = ENV_TEMPLATE.lines().map(|line| {
        if line.starts_with('#') || line.trim().is_empty() {
            line.to_string()
        } else {
            match line.split('=').next() {
                Some("SERVER_PORT") => format!("SERVER_PORT={}", new_server_port),
                Some("RCON_PORT") => format!("RCON_PORT={}", new_rcon_port),
                Some("SFTP_SERVER_PORT") => format!("SFTP_SERVER_PORT={}", new_sftp_port),
                Some("MINECRAFT_SERVER_SERVICE") => 
                    format!("MINECRAFT_SERVER_SERVICE=minecraft_server_{}", new_stack_id),
                Some("MINECRAFT_SERVER_VOLUME") => 
                    format!("MINECRAFT_SERVER_VOLUME=minecraft_server_{}", new_stack_id),
                Some("MINECRAFT_SERVER_NETWORK") => 
                    format!("MINECRAFT_SERVER_NETWORK=minecraft_server_{}", new_stack_id),
                Some("SFTP_SERVER_SERVICE") => 
                    format!("SFTP_SERVER_SERVICE=sftp_server_{}", new_stack_id),
                _ => line.to_string()
            }
        }
    }).collect::<Vec<String>>().join("\n");

    // Write files
    fs::write(new_stack_dir.join(".env"), new_content)
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to write .env file: {}", e)))?;
    
    fs::write(new_stack_dir.join("compose.yaml"), COMPOSE_TEMPLATE)
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to write compose.yaml: {}", e)))?;

    // Start the containers
    let output = Command::new("docker")
        .args([
            "compose",
            "-f",
            new_stack_dir.join("compose.yaml").to_str().unwrap(),
            "up",
            "-d"
        ])
        .output()
        .await
        .map_err(|e| CreateStackError::DockerError(format!("Failed to execute docker compose: {}", e)))?;

    if !output.status.success() {
        fs::remove_dir_all(&new_stack_dir)
            .map_err(|e| CreateStackError::FileSystemError(format!("Failed to cleanup failed stack: {}", e)))?;
        
        return Err(CreateStackError::DockerError(
            "Failed to start containers. Stack creation rolled back".to_string()
        ))?;
    }

    Ok(HttpResponse::Created().json(json!({
        "stack_id": new_stack_id.to_string(),
        "ports": {
            "minecraft_server": new_server_port.to_string(),
            "rcon": new_rcon_port.to_string(),
            "sftp_server": new_sftp_port.to_string()
        }
    })))
}

#[post("/stacks")]
pub async fn create_stack() -> Result<HttpResponse, Error> {
    create_stack_impl().await
}