use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_cors::Cors;
use actix_web::{
    cookie::{Key, SameSite},
    error::InternalError,
    middleware, middleware::Logger, 
    dev::ServiceRequest, web, App, HttpResponse, Error, HttpServer, Responder};
use actix_web_httpauth::{extractors::AuthenticationError, extractors::bearer::BearerAuth, middleware::HttpAuthentication};

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod products;
mod users;
mod errors;
pub mod types;

// API overview
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to API.
    "#
    )
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    eprintln!("{:?}", credentials);

    Ok(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("environment variable: DATABASE_URL");
    let host = env::var("HOST").expect("environment variable: HOST");
    let port = env::var("PORT").expect("environment variable: PORT");
    let addr = format!("{}:{}", host, port);
    
    let signing_key = Key::generate();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create connections pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            // .wrap(HttpAuthentication::bearer(validator))
            .wrap(Cors::permissive())
            .route("/health_check", web::get().to(health_check))
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    signing_key.clone(),
                )
                .cookie_http_only(false)
                .cookie_same_site(SameSite::Strict)
                .build(),
            )
            .route("/login", web::post().to(users::handlers::login))
            .service(
                web::scope("/api").service(web::scope("/v1")
                    .configure(users::handlers::config)
                    .configure(products::handlers::config)),
            )
            .route("/", web::get().to(index))
    })
    .bind(addr)?
    .run()
    .await
}