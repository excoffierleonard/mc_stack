use actix_files as fs;
use actix_web::{delete, post, put, web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;

async fn create_stack() -> impl Responder {
    let output = Command::new("./scripts/create_stack.sh")
        .output()
        .expect("Failed to execute script");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    HttpResponse::Ok().body(stdout)
}

#[delete("/{stack_id}")]
async fn delete_stack(path: web::Path<String>) -> impl Responder {
    let stack_id = path.into_inner();
    let output = Command::new("./scripts/delete_stack.sh")
        .arg(&stack_id)
        .output()
        .expect("Failed to execute script");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    HttpResponse::Ok().body(stdout)
}

#[put("/{stack_id}")]
async fn start_stack(path: web::Path<String>) -> impl Responder {
    let stack_id = path.into_inner();
    let output = Command::new("./scripts/start_stack.sh")
        .arg(&stack_id)
        .output()
        .expect("Failed to execute script");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    HttpResponse::Ok().body(stdout)
}

#[post("/{stack_id}")]
async fn stop_stack(path: web::Path<String>) -> impl Responder {
    let stack_id = path.into_inner();
    let output = Command::new("./scripts/stop_stack.sh")
        .arg(&stack_id)
        .output()
        .expect("Failed to execute script");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    HttpResponse::Ok().body(stdout)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/create").route(web::post().to(create_stack)))
            .service(delete_stack)
            .service(start_stack)
            .service(stop_stack)
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
