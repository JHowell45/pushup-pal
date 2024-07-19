use actix_utils::future::{ready, Ready};
use actix_web::{dev, error, web, FromRequest, HttpRequest, Responder, Result};

pub struct MiniJinjaRenderer {
    templ_env: web::Data<minijinja_autoreload::AutoReloader>,
}

impl MiniJinjaRenderer {
    pub fn render(
        &self,
        templ: &str,
        ctx: impl Into<minijinja::value::Value>,
    ) -> Result<impl Responder> {
        self.templ_env
            .acquire_env()
            .map_err(|_| error::ErrorInternalServerError("Could not acquire template env"))?
            .get_template(templ)
            .map_err(|_| error::ErrorInternalServerError("Could not find template"))?
            .render(ctx.into())
            .map(web::Html::new)
            .map_err(|err| {
                log::error!("{err}");
                error::ErrorInternalServerError("template error")
            })
    }
}

impl FromRequest for MiniJinjaRenderer {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut dev::Payload) -> Self::Future {
        let templ_env = <web::Data<minijinja_autoreload::AutoReloader>>::extract(req)
            .into_inner()
            .unwrap();

        ready(Ok(Self { templ_env }))
    }
}
