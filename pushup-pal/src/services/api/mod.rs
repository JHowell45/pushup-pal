mod pushups;

use actix_web::web;

pub fn pushup_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pushup")
            .route("", web::get().to(pushups::all))
            .route("/", web::get().to(pushups::all))
            .route("/{amount}", web::post().to(pushups::post))
            .route("/<session_id>", web::get().to(pushups::get)),
    );
}
