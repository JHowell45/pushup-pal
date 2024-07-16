use actix_web::{error, Result};
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use database::actions;
use serde::Deserialize;

#[macro_use]
extern crate diesel;

mod database;
mod services;

use crate::database::initialize_db_pool;

static IP: &str = "127.0.0.1";
static PORT: u16 = 9000;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or(std::env::var("LOG_LEVEL").unwrap_or(String::from("info"))),
    );

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
    })
    .bind((IP, PORT))?
    .run()
    .await
}
