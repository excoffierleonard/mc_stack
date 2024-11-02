use actix_web::{delete, web, Error, HttpResponse};
use crate::utils::script_executor::execute_script;

#[delete("/{stack_id}")]
pub async fn delete_stack(stack_id: web::Path<String>) -> Result<HttpResponse, Error> {
    execute_script("delete_stack", Some(&stack_id)).await
}