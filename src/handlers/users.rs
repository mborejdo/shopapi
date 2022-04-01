use actix_session::Session;
use crate::errors::ServiceError;
use crate::{
    models::users::{User, UserInput, Credentials},
    types::PostgresPool,
};
use actix_web::{web, HttpResponse, Responder};

async fn find_all(
    pool: web::Data<PostgresPool>
) -> Result<impl Responder, ServiceError> {
    let result = User::find_all(pool.get_ref()).await;
    match result {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        _ => Err(ServiceError::BadRequest("Error trying to read all users from database".to_string())),
    }
}

async fn create(
    session: Session, 
    input: web::Json<UserInput>, 
    pool: web::Data<PostgresPool>
) -> Result<impl Responder, ServiceError> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);
    match user_id {
        Some(_id) => {
            session.renew();
            let result = User::create(input.into_inner(), pool.get_ref()).await;
            match result {
                Ok(users) => {
                    Ok(HttpResponse::Ok().json(users))
                },
                _ => Err(ServiceError::BadRequest("Error trying to create new user".to_string())),
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

async fn find_by_id(
    id: web::Path<i32>, 
    pool: web::Data<PostgresPool>
) -> impl Responder {
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
) -> Result<impl Responder, ServiceError> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(_userid) => {
            session.renew();
            let result = User::update(id.into_inner(), input.into_inner(), pool.get_ref()).await;
            match result {
                Ok(users) => Ok(HttpResponse::Ok().json(users)),
                _ => Ok(HttpResponse::NotFound().body("User not found")),
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

async fn delete(
    session: Session, 
    id: web::Path<i32>, 
    db_pool: web::Data<PostgresPool>
) -> Result<impl Responder, ServiceError> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(_userid) => {
            session.renew();
            let result = User::delete(id.into_inner(), db_pool.get_ref()).await;
            match result {
                Ok(rows) => {
                    if rows > 0 {
                        Ok(HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows)))
                    } else {
                        Ok(HttpResponse::NotFound().body("User not found"))
                    }
                }
                _ => Err(ServiceError::InternalServerError),
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

pub async fn login(
    session: Session,
    credentials: web::Json<Credentials>,
    db_pool: web::Data<PostgresPool>
) -> Result<impl Responder, ServiceError> {
    let credentials = credentials.into_inner();

    let user = match User::authenticate(credentials, db_pool.get_ref()).await {
        Ok(user) => {
            session.insert("user_id", user.id).unwrap();
            user
        },
        Err(_err) => return Err(ServiceError::Unauthorized),
    };

    Ok(format!("Welcome!, {:?}", user))
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