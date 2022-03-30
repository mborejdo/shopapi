use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_cors::Cors;
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger, 
    web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::{extractors::bearer::BearerAuth};
use actix_files::Files;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod products;
mod users;
mod auth;
// mod utils;
mod errors;
pub mod types;


// MOVE ROUTES
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to API.
    "#
    )
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// async fn validator(
//     req: ServiceRequest,
//     credentials: BearerAuth,
// ) -> Result<ServiceRequest, Error> {
//     eprintln!("{:?}", credentials);

//     Ok(req)
// }

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
        .expect("Failed to create pg pool");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(Logger::default())
            // .wrap(HttpAuthentication::bearer(validator))
            .wrap(Cors::permissive())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    signing_key.clone(),
                )
                .cookie_http_only(false)
                .cookie_same_site(SameSite::Strict)
                .build(),
            )
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/login", web::post().to(users::handlers::login))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .configure(users::handlers::config)
                        .configure(products::handlers::config)
                ),
            )
            .service(
                web::scope("/static").default_service(
                    Files::new("", "./static")
                        .index_file("index.html")
                        .use_last_modified(true),
                ),
            )
    })
    .bind(addr)?
    .run()
    .await
}