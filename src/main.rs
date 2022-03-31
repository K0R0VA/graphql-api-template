mod api;

use actix_web::{App, HttpServer, web::Data};
use sqlx::PgPool;
use crate::api::api_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let env = std::env::var("DATABASE_URL").expect("database url is not set");
    let database = Data::new(PgPool::connect(&env).await.expect("cannot create database pool"));
    HttpServer::new(move || App::new()
        .app_data(database.clone())
        .configure(api_config)
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}