use crate::errors::ServiceError;
use actix_web::{web, Responder};
use crate::{
    models::search::meili_search,
};
pub async fn search(
    query: web::Path<String>,
) -> Result<impl Responder, ServiceError> {
    let data = meili_search(&query).await;
    match data {
        Ok(documents) => Ok(format!("Found:!, {:?}", documents)),
        _ => Err(ServiceError::BadRequest(
            "Error searching".to_string(),
        )),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/search/{query}")
            .route(web::get().to(search)),
    );
}
