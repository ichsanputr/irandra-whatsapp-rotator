mod module;
mod dto;
mod model;

use actix_web::{web, App, HttpServer};
use sqlx::MySqlPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPool::connect(&db_url)
        .await
        .expect("Failed to connect to MySQL");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/auth")
                .service(module::auth::login)
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}