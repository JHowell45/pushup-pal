use actix_web::{error, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use uuid::Uuid;

use crate::database::DbPool;

#[derive(Deserialize)]
pub struct GetInfo {
    id: Uuid,
}

pub async fn get(pool: web::Data<DbPool>, path: web::Path<GetInfo>) -> Result<impl Responder> {
    let pushup_session = web::block(move || {
        let mut conn = pool.get()?;

        crate::database::actions::get_pushup_session(&mut conn, path.id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match pushup_session {
        Some(session) => HttpResponse::Ok().json(session),
        None => HttpResponse::NotFound().body(""),
    })
}


pub async fn all(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let pushup_sessions = web::block(move || {
        let mut conn = pool.get()?;

        crate::database::actions::get_pushup_sessions(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(pushup_sessions))
}