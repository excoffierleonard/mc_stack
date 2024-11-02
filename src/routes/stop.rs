use actix_web::{post, web, Error, HttpResponse};
use crate::utils::script_executor::execute_script;

#[post("/{stack_id}")]
pub async fn stop_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    execute_script("stop_stack", Some(&stack_id)).await
}