use crate::errors::ServiceError;
use crate::{
    models::images::{Image, ImageInput},
    types::PostgresPool,
};
use actix_image::Session;
use actix_web::{web, HttpResponse, Responder};

async fn find_all(
    session: Session,
    pool: web::Data<PostgresPool>,
) -> Result<impl Responder, ServiceError> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(_userid) => {
            session.renew();
            let result = Image::find_all(pool.get_ref()).await;
            match result {
                Ok(images) => Ok(HttpResponse::Ok().json(images)),
                _ => Err(ServiceError::BadRequest(
                    "Error trying to read all images from database".to_string(),
                )),
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

async fn create(
    session: Session,
    input: web::Json<ImageInput>,
    pool: web::Data<PostgresPool>,
) -> Result<impl Responder, ServiceError> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(_id) => {
            session.renew();
            let result = Image::create(input.into_inner(), pool.get_ref()).await;
            match result {
                Ok(image) => Ok(HttpResponse::Ok().json(image)),
                _ => Err(ServiceError::BadRequest(
                    "Error trying to create new image".to_string(),
                )),
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

async fn find_by_id(id: web::Path<i32>, pool: web::Data<PostgresPool>) -> impl Responder {
    let result = Image::find_by_id(id.into_inner(), pool.get_ref()).await;
    match result {
        Ok(image) => HttpResponse::Ok().json(image),
        _ => HttpResponse::NotFound().body("Session not found"),
    }
}

async fn update(
    session: Session,
    id: web::Path<i32>,
    input: web::Json<ImageInput>,
    pool: web::Data<PostgresPool>,
) -> Result<impl Responder, ServiceError> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);
    match user_id {
        Some(_userid) => {
            session.renew();
            let result = Image::update(id.into_inner(), input.into_inner(), pool.get_ref()).await;
            match result {
                Ok(image) => Ok(HttpResponse::Ok().json(image)),
                _ => Ok(HttpResponse::NotFound().body("Session not found")),
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

async fn delete(
    session: Session,
    id: web::Path<i32>,
    db_pool: web::Data<PostgresPool>,
) -> Result<impl Responder, ServiceError> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);
    match user_id {
        Some(_userid) => {
            session.renew();
            let result = Image::delete(id.into_inner(), db_pool.get_ref()).await;
            match result {
                Ok(rows) => {
                    if rows > 0 {
                        Ok(HttpResponse::Ok()
                            .body(format!("Successfully deleted {} record(s)", rows)))
                    } else {
                        Ok(HttpResponse::NotFound().body("Session not found"))
                    }
                }
                _ => Err(ServiceError::InternalServerError),
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/images")
            .route(web::get().to(find_all))
            .route(web::post().to(create)),
    );
    cfg.service(
        web::resource("/images/{id}")
            .route(web::get().to(find_by_id))
            .route(web::put().to(update))
            .route(web::delete().to(delete)),
    );
}
