use actix_web::{error, web, Responder, Result};

use crate::database::DbPool;

use super::renderer::MiniJinjaRenderer;

pub async fn index(
    templ_env: MiniJinjaRenderer,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let count = web::block(move || {
        let mut conn = pool.get()?;

        crate::database::actions::get_todays_pushup_total(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    templ_env.render(
        "index.html",
        minijinja::context! {
            dailyCount => count
        },
    )
}
