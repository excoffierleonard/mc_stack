use actix_files as fs;
use actix_web::{
    delete, error::Error, get, middleware::{Compress, Logger},
    post, put, web, App, HttpResponse, HttpServer,
};
use env_logger::Env;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct ApiResponse {
    status: String,
    message: String,
}

impl ApiResponse {
    fn success(message: String) -> Self {
        Self {
            status: "success".to_string(),
            message,
        }
    }

    fn error(message: String) -> Self {
        Self {
            status: "error".to_string(),
            message,
        }
    }
}

async fn execute_script(script_name: &str, args: Option<&str>) -> Result<HttpResponse, Error> {
    let script_path = format!("./scripts/{}.sh", script_name);
    let result = Command::new(&script_path)
        .args(args)
        .output()
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to execute {}: {}", 
                script_name, e
            ))
        })?;

    let response = if result.status.success() {
        ApiResponse::success(String::from_utf8_lossy(&result.stdout).to_string())
    } else {
        ApiResponse::error(String::from_utf8_lossy(&result.stderr).to_string())
    };

    Ok(HttpResponse::Ok().json(response))
}

// Route handlers become much simpler
async fn create_stack() -> Result<HttpResponse, Error> {
    execute_script("create_stack", None).await
}

#[delete("/{stack_id}")]
async fn delete_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    execute_script("delete_stack", Some(&stack_id)).await
}

#[put("/{stack_id}")]
async fn start_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    execute_script("start_stack", Some(&stack_id)).await
}

#[post("/{stack_id}")]
async fn stop_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    execute_script("stop_stack", Some(&stack_id)).await
}

#[get("/list")]
async fn list_stacks() -> Result<HttpResponse, Error> {
    execute_script("list_stacks", None).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(web::resource("/create").route(web::post().to(create_stack)))
                    .service(delete_stack)
                    .service(start_stack)
                    .service(stop_stack)
                    .service(list_stacks),
            )
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .workers(2)
    .run()
    .await
}
