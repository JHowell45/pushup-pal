use actix_web::web;

mod routes;

use routes::get;

pub fn pushup_scope(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get));
}
