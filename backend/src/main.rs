use actix_web::{web, App, HttpServer, middleware::DefaultHeaders, http::header};
use actix_cors::Cors;

mod handlers;
mod models;
mod db;

use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:password@db:3306/webauthn".to_string());

    let pool = db::create_pool(&database_url).await.expect("Failed to create database pool");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(DefaultHeaders::new().add((header::CONTENT_TYPE, "application/json")))
            .route("/", web::get().to(index))
            .route("/register/request", web::post().to(register_request))
            .route("/register/response", web::post().to(register_response))
            .route("/login/request", web::post().to(login_request))
            .route("/login/response", web::post().to(login_response))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}