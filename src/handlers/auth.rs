use crate::errors::ServiceError;
use crate::{
    models::auth::{Credentials, Auth},
    types::PostgresPool,
};
use actix_session::Session;
use actix_web::{web, Responder};

pub async fn login(
    session: Session,
    credentials: web::Json<Credentials>,
    db_pool: web::Data<PostgresPool>,
) -> Result<impl Responder, ServiceError> {
    let credentials = credentials.into_inner();

    let user = match Auth::authenticate(credentials, db_pool.get_ref()).await {
        Ok(user) => {
            session.insert("user_id", user.id).unwrap();
            user
        }
        Err(_err) => return Err(ServiceError::Unauthorized),
    };

    Ok(format!("Welcome!, {:?}", user))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::post().to(login)),
    );
    cfg.service(
        web::resource("/auth/login")
            .route(web::post().to(login)),
    );
}
