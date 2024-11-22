use actix_web::{web, HttpResponse, Result};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "website/"]
pub struct Assets;

async fn serve_file(path: web::Path<String>) -> Result<HttpResponse> {
    let path = path.into_inner();
    let path = if path.is_empty() {
        "index.html".to_string()
    } else {
        path
    };

    if let Some(content) = Assets::get(path.as_str()) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();

        Ok(HttpResponse::Ok()
            .content_type(mime.as_ref())
            .body(content.data.to_vec()))
    } else {
        // SPA fallback to index.html
        if let Some(content) = Assets::get("index.html") {
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(content.data.to_vec()))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/{filename:.*}", web::get().to(serve_file));
}
