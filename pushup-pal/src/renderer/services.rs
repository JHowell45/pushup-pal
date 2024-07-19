use std::collections::HashMap;

use actix_web::{web, Responder, Result};

use super::renderer::MiniJinjaRenderer;

pub async fn index(
    templ_env: MiniJinjaRenderer,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder> {
    let count = match query.get("count") {
        Some(c) => 100,
        None => 0,
    };
    templ_env.render(
        "index.html",
        minijinja::context! {
            dailyCount => count
        },
    )
}
