use actix_web::{post, Error, HttpResponse};
use crate::utils::script_executor::execute_script;

#[post("/create")]
pub async fn create_stack() -> Result<HttpResponse, Error> {
    execute_script("create_stack", None).await
}