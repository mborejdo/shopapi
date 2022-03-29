use actix_session::{Session};

use crate::{
    users::models::{User, UserInput, Credentials},
    types::PostgresPool,
};
use actix_web::{web, HttpResponse, Error, Responder, error::InternalError};

async fn find_all(pool: web::Data<PostgresPool>) -> impl Responder {
    let result = User::find_all(pool.get_ref()).await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        _ => HttpResponse::BadRequest().body("Error trying to read all users from database"),
    }
}

async fn create(session: Session, input: web::Json<UserInput>, pool: web::Data<PostgresPool>) -> impl Responder {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);
    match user_id {
        Some(id) => {
            session.renew();
            let result = User::create(input.into_inner(), pool.get_ref()).await;
            match result {
                Ok(users) => HttpResponse::Ok().json(users),
                _ => HttpResponse::BadRequest().body("Error trying to create new user"),
            }
        }
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

async fn find_by_id(id: web::Path<i32>, pool: web::Data<PostgresPool>) -> impl Responder {
    let result = User::find_by_id(id.into_inner(), pool.get_ref()).await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        _ => HttpResponse::NotFound().body("User not found"),
    }
}

async fn update(
    session: Session, 
    id: web::Path<i32>,
    input: web::Json<UserInput>,
    pool: web::Data<PostgresPool>,
) -> impl Responder {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(xid) => {
            session.renew();
            let result = User::update(id.into_inner(), input.into_inner(), pool.get_ref()).await;
            match result {
                Ok(users) => HttpResponse::Ok().json(users),
                _ => HttpResponse::NotFound().body("User not found"),
            }
        }
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

async fn delete(session: Session, id: web::Path<i32>, db_pool: web::Data<PostgresPool>) -> impl Responder {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(xid) => {
            session.renew();
            let result = User::delete(id.into_inner(), db_pool.get_ref()).await;
            match result {
                Ok(rows) => {
                    if rows > 0 {
                        HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows))
                    } else {
                        HttpResponse::NotFound().body("User not found")
                    }
                }
                _ => HttpResponse::InternalServerError().body("Failed to delete user"),
            }
        }
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

pub async fn login(
    credentials: web::Json<Credentials>,
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();

    match User::authenticate(credentials) {
        Ok(user) => session.insert("user_id", user.id).unwrap(),
        Err(err) => return Err(InternalError::from_response("", err).into()),
    };

    Ok("Welcome!")
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(find_all))
            .route(web::post().to(create)),
    );
    cfg.service(
        web::resource("/users/{id}")
            .route(web::get().to(find_by_id))
            .route(web::put().to(update))
            .route(web::delete().to(delete)),
    );
}