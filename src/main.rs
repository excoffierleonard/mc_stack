use actix_files as fs;
use actix_web::{
    delete,
    error::Error,
    get,
    middleware::{Compress, Logger},
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

async fn execute_script(script_name: &str, args: Option<&str>) -> Result<HttpResponse, Error> {
    let mut command = Command::new(format!("./scripts/{}.sh", script_name));

    if let Some(arg) = args {
        command.arg(arg);
    }

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(HttpResponse::Ok().json(ApiResponse {
                    status: "success".to_string(),
                    message: String::from_utf8_lossy(&output.stdout).to_string(),
                }))
            } else {
                Ok(HttpResponse::InternalServerError().json(ApiResponse {
                    status: "error".to_string(),
                    message: String::from_utf8_lossy(&output.stderr).to_string(),
                }))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse {
            status: "error".to_string(),
            message: format!("Failed to execute {}: {}", script_name, e),
        })),
    }
}

async fn create_stack() -> Result<HttpResponse, Error> {
    execute_script("create_stack", None).await
}

#[delete("/{stack_id}")]
async fn delete_stack(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let stack_id = path.into_inner();
    execute_script("delete_stack", Some(&stack_id)).await
}

#[put("/{stack_id}")]
async fn start_stack(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let stack_id = path.into_inner();
    execute_script("start_stack", Some(&stack_id)).await
}

#[post("/{stack_id}")]
async fn stop_stack(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let stack_id = path.into_inner();
    execute_script("stop_stack", Some(&stack_id)).await
}

#[get("/list")]
async fn list_stacks() -> Result<HttpResponse, Error> {
    execute_script("list_stacks", None).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger with default level 'info'
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            // Enable compression
            .wrap(Compress::default())
            // Enable logging
            .wrap(Logger::default())
            // API routes
            .service(
                web::scope("/api/v1")
                    .service(web::resource("/create").route(web::post().to(create_stack)))
                    .service(delete_stack)
                    .service(start_stack)
                    .service(stop_stack)
                    .service(list_stacks),
            )
            // Static files
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .workers(2)
    .run()
    .await
}
