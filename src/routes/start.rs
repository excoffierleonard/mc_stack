use actix_web::{put, web, Error, HttpResponse};
use crate::utils::script_executor::execute_script;

#[put("/{stack_id}")]
pub async fn start_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    execute_script("start_stack", Some(&stack_id)).await
}