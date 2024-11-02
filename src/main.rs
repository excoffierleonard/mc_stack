use actix_files as fs;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use env_logger::Env;
use num_cpus;

mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!("Starting server at http://0.0.0.0:8080");

    let num_workers = num_cpus::get();

    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(routes::create::create_stack)
                    .service(routes::delete::delete_stack)
                    .service(routes::start::start_stack)
                    .service(routes::stop::stop_stack)
                    .service(routes::list::list_stacks),
            )
            .service(fs::Files::new("/", "web/").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .workers(num_workers)
    .run()
    .await
}