use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        Welcome to API.
    "#,
    )
}
