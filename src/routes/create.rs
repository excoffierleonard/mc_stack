// src/routes/create.rs
use actix_web::{post, Error, HttpResponse, ResponseError};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::fs;
use std::fmt;
use tokio::process::Command;
use regex::Regex;
use num_cpus;

const INCREMENT: i32 = 3;

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
            CreateStackError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
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

// Rest of the code remains the same but now you can use ? operator directly
struct EnvConfig {
    server_port: i32,
    rcon_port: i32,
    sftp_port: i32,
}

async fn get_base_directories() -> Result<(PathBuf, PathBuf), CreateStackError> {
    let current_exe = std::env::current_exe()
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to get current path: {}", e)))?;
    
    let base_dir = current_exe
        .parent()
        .ok_or_else(|| CreateStackError::FileSystemError("Failed to find executable directory".to_string()))?;

    let stacks_dir = base_dir.join("stacks");
    let template_dir = base_dir.join("template");

    if !stacks_dir.exists() {
        return Err(CreateStackError::ValidationError("Stacks directory does not exist".to_string()));
    }
    if !template_dir.exists() || !template_dir.join(".env").exists() || !template_dir.join("compose.yaml").exists() {
        return Err(CreateStackError::ValidationError("Template files not found".to_string()));
    }

    Ok((stacks_dir, template_dir))
}

fn parse_env_file(env_path: &Path) -> Result<EnvConfig, CreateStackError> {
    let content = fs::read_to_string(env_path)
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to read env file: {}", e)))?;

    let get_port = |var_name: &str| -> Result<i32, CreateStackError> {
        let re = Regex::new(&format!(r"^{}=(\d+)", var_name)).unwrap();
        content
            .lines()
            .find_map(|line| re.captures(line))
            .and_then(|cap| cap[1].parse().ok())
            .ok_or_else(|| CreateStackError::ValidationError(format!("{} not found in env file", var_name)))
    };

    Ok(EnvConfig {
        server_port: get_port("SERVER_PORT")?,
        rcon_port: get_port("RCON_PORT")?,
        sftp_port: get_port("SFTP_SERVER_PORT")?,
    })
}

async fn create_stack_impl() -> Result<HttpResponse, Error> {
    let (stacks_dir, template_dir) = get_base_directories().await?;

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

    // Rest of the implementation remains the same but now you can use ? operator directly
    let new_stack_id = highest_number + 1;
    let new_stack_dir = stacks_dir.join(format!("stack_{}", new_stack_id));

    let env_config = parse_env_file(&template_dir.join(".env"))?;
    
    let new_server_port = env_config.server_port + new_stack_id * INCREMENT;
    let new_rcon_port = env_config.rcon_port + new_stack_id * INCREMENT;
    let new_sftp_port = env_config.sftp_port + new_stack_id * INCREMENT;

    // Create new stack directory and copy files
    fs::create_dir_all(&new_stack_dir)
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to create stack directory: {}", e)))?;

    for file in &["compose.yaml", ".env"] {
        fs::copy(
            template_dir.join(file),
            new_stack_dir.join(file),
        ).map_err(|e| CreateStackError::FileSystemError(format!("Failed to copy {}: {}", file, e)))?;
    }

    // Update .env file with correct values
    let env_path = new_stack_dir.join(".env");
    let content = fs::read_to_string(&env_path)
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to read .env file: {}", e)))?;

    let new_content = content.lines().map(|line| {
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

    fs::write(&env_path, new_content)
        .map_err(|e| CreateStackError::FileSystemError(format!("Failed to update .env file: {}", e)))?;

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

    Ok(HttpResponse::Ok().json(json!({
        "message": format!("Stack {} has been successfully created", new_stack_id),
        "data": {
            "stack_id": new_stack_id.to_string(),
            "ports": {
                "minecraft_server": new_server_port.to_string(),
                "rcon": new_rcon_port.to_string(),
                "sftp_server": new_sftp_port.to_string()
            }
        }
    })))
}

#[post("/create")]
pub async fn create_stack() -> Result<HttpResponse, Error> {
    create_stack_impl().await
}