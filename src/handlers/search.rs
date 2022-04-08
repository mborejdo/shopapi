use crate::errors::ServiceError;
use actix_web::{web, Responder};
use crate::{
    models::search::meili_search,
};
pub async fn search(
) -> Result<impl Responder, ServiceError> {

    let data = meili_search("can").await;

    Ok(format!("Welcome!, {:?}", data))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/search")
            .route(web::get().to(search)),
    );
}
