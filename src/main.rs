use actix_cors::Cors;
use actix_files::Files;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web, App, HttpServer,
};

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod auth;
mod errors;
mod handlers;
mod models;
mod routes;

pub mod types;

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
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            // .wrap(HttpAuthentication::bearer(validator))
            .wrap(Cors::permissive())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), signing_key.clone())
                    .cookie_http_only(false)
                    .cookie_same_site(SameSite::Strict)
                    .build(),
            )
            .route("/", web::get().to(routes::index))
            .route("/login", web::post().to(handlers::users::login))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .configure(handlers::users::config)
                        .configure(handlers::products::config)
                        .configure(handlers::orders::config),
                ),
            )
            .configure(handlers::auth::config)
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
