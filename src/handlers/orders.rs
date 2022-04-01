use crate::errors::ServiceError;
use crate::{
    models::orders::{Order, OrderInput},
    types::PostgresPool,
};
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};

async fn find_all(pool: web::Data<PostgresPool>) -> Result<impl Responder, ServiceError> {
    let result = Order::find_all(pool.get_ref()).await;
    match result {
        Ok(orders) => Ok(HttpResponse::Ok().json(orders)),
        _ => Err(ServiceError::BadRequest(
            "Error trying to read all orders from database".to_string(),
        )),
    }
}

async fn create(
    session: Session,
    input: web::Json<OrderInput>,
    pool: web::Data<PostgresPool>,
) -> Result<impl Responder, ServiceError> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(_id) => {
            session.renew();
            let result = Order::create(input.into_inner(), pool.get_ref()).await;
            match result {
                Ok(order) => Ok(HttpResponse::Ok().json(order)),
                _ => Err(ServiceError::BadRequest(
                    "Error trying to create new order".to_string(),
                )),
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

async fn find_by_id(id: web::Path<i32>, pool: web::Data<PostgresPool>) -> impl Responder {
    let result = Order::find_by_id(id.into_inner(), pool.get_ref()).await;
    match result {
        Ok(order) => HttpResponse::Ok().json(order),
        _ => HttpResponse::NotFound().body("Order not found"),
    }
}

async fn update(
    session: Session,
    id: web::Path<i32>,
    input: web::Json<OrderInput>,
    pool: web::Data<PostgresPool>,
) -> Result<impl Responder, ServiceError> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);
    match user_id {
        Some(_userid) => {
            session.renew();
            let result = Order::update(id.into_inner(), input.into_inner(), pool.get_ref()).await;
            match result {
                Ok(order) => Ok(HttpResponse::Ok().json(order)),
                _ => Ok(HttpResponse::NotFound().body("Order not found")),
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
            let result = Order::delete(id.into_inner(), db_pool.get_ref()).await;
            match result {
                Ok(rows) => {
                    if rows > 0 {
                        Ok(HttpResponse::Ok()
                            .body(format!("Successfully deleted {} record(s)", rows)))
                    } else {
                        Ok(HttpResponse::NotFound().body("Order not found"))
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
        web::resource("/orders")
            .route(web::get().to(find_all))
            .route(web::post().to(create)),
    );
    cfg.service(
        web::resource("/orders/{id}")
            .route(web::get().to(find_by_id))
            .route(web::put().to(update))
            .route(web::delete().to(delete)),
    );
}
