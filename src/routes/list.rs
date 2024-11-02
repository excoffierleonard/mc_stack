use actix_web::{get, Error, HttpResponse};
use crate::utils::script_executor::execute_script;

#[get("/list")]
pub async fn list_stacks() -> Result<HttpResponse, Error> {
    execute_script("list_stacks", None).await
}