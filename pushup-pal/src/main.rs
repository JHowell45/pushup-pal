use actix_web::{error, Result};
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use database::actions;
use serde::Deserialize;

#[macro_use]
extern crate diesel;

mod database;
mod services;

use crate::database::{initialize_db_pool, DbPool};

static IP: &str = "127.0.0.1";
static PORT: u16 = 9000;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let latest_pushup_session = web::block(move || {
        let mut conn = pool.get()?;
        // actions::get_todays_pushup_total(&mut conn, Utc::now().date_naive())
        // actions::get_first_pushup_session(&mut conn)
        actions::get_latest_pushup_session(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(latest_pushup_session))
}

#[derive(Deserialize)]
struct UpdateInfo {
    amount: i32,
}

#[post("/add/{amount}")]
async fn update(path: web::Path<UpdateInfo>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let created_pushup_session = web::block(move || {
        let mut conn = pool.get()?;
        actions::insert_new_pushup_session(&mut conn, path.amount)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(created_pushup_session))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    println!("Server running on: http://{IP}:{PORT}");
    HttpServer::new(move || {
        App::new()
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            // add request logger middleware
            .wrap(middleware::Logger::default())
            .service(web::scope("/api").configure(services::api::pushup_scope))
            .service(index)
            .service(update)
    })
    .bind((IP, PORT))?
    .run()
    .await
}
