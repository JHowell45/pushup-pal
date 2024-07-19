use std::path::PathBuf;

use actix_web::{
    get,
    http::StatusCode,
    middleware::{self, ErrorHandlers},
    web, App, HttpResponse, HttpServer, Responder,
};
use minijinja::path_loader;
use minijinja_autoreload::AutoReloader;

#[macro_use]
extern crate diesel;

mod database;
mod renderer;
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

    // If TEMPLATE_AUTORELOAD is set, then the path tracking is enabled.
    let enable_template_autoreload = std::env::var("TEMPLATE_AUTORELOAD").as_deref() == Ok("true");

    if enable_template_autoreload {
        log::info!("template auto-reloading is enabled");
    } else {
        log::info!(
            "template auto-reloading is disabled; run with TEMPLATE_AUTORELOAD=true to enable"
        );
    }

    // The closure is invoked every time the environment is outdated to recreate it.
    let tmpl_reloader = AutoReloader::new(move |notifier| {
        let mut env: minijinja::Environment<'static> = minijinja::Environment::new();

        let tmpl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");

        // if watch_path is never called, no fs watcher is created
        if enable_template_autoreload {
            notifier.watch_path(&tmpl_path, true);
        }

        env.set_loader(path_loader(tmpl_path));

        Ok(env)
    });

    let tmpl_reloader = web::Data::new(tmpl_reloader);

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    log::info!("Server running on: http://{IP}:{PORT}");
    HttpServer::new(move || {
        App::new()
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            .app_data(tmpl_reloader.clone())
            .service(web::resource("/").route(web::get().to(crate::renderer::services::index)))
            // .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found))
            // add request logger middleware
            .wrap(middleware::Logger::default())
            .service(web::scope("/api").configure(services::api::pushup_scope))
    })
    .bind((IP, PORT))?
    .run()
    .await
}
