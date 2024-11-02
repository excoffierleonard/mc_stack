use actix_web::{HttpResponse, Error};
use std::process::Command;

pub async fn execute_script(script_name: &str, args: Option<&str>) -> Result<HttpResponse, Error> {
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

    if result.status.success() {
        let output = String::from_utf8(result.stdout)
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!(
                "Invalid UTF-8 in stdout: {}", e
            )))?;
        
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(output))
    } else {
        let error = String::from_utf8(result.stderr)
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!(
                "Invalid UTF-8 in stderr: {}", e
            )))?;
        
        Ok(HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(error))
    }
}