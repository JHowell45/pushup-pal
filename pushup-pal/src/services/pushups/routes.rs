use actix_web::{HttpResponse, Responder, Result};
use std::collections::HashMap;

async fn get() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(HashMap::from([("hello", "Test")])))
}
