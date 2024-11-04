// Copyright (C) 2024 Leonard Excoffier

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use env_logger::Env;
use num_cpus;

mod routes;
mod website;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!("Starting server at http://0.0.0.0:8080");

    let num_workers = num_cpus::get();

    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            // API routes
            .service(
                web::scope("/api/v1")
                    .service(routes::create::create_stack)
                    .service(routes::delete::delete_stack)
                    .service(routes::status::update_stack_status)
                    .service(routes::list::list_stacks),
            )
            // Static web files
            .configure(website::config)
    })
    .bind("0.0.0.0:8080")?
    .workers(num_workers)
    .run()
    .await
}