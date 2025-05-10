mod dto;
mod model;
mod module;
mod helper;

use actix_web::{web, App, HttpServer};

use actix_cors::Cors;
use sqlx::MySqlPool;
use std::env;
use tracing_appender::rolling;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Log rotation: daily logs in logs/paypal.log
    let file_appender = rolling::daily("logs", "log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            EnvFilter::new("info").add_directive("actix_server::=off".parse().unwrap()),
        )
        .with(fmt::layer().json().with_writer(file_writer))
        .init();

    // Env
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Database
    let pool = MySqlPool::connect(&db_url)
        .await
        .expect("Failed to connect to MySQL");

    // Serve
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/auth").service(module::auth::login))
            .service(
                web::scope("/operator")
                    .service(module::operator::add_operator)
                    .service(module::operator::list_operator)
                    .service(module::operator::list_option)
                    .service(module::operator::delete_operator)
                    .service(module::operator::update_operator)
                    .service(module::operator::get_operator_by_id),
            )
            .service(
                web::scope("/campaign")
                    .service(module::campaign::add_campaign)
                    .service(module::campaign::list_campaign)
                    .service(module::campaign::delete_campaign)
                    .service(module::campaign::update_campaign)
                    .service(module::campaign::get_campaign_visitor)
                    .service(module::campaign::get_campaign_chart)
                    .service(module::campaign::get_campaign_dashboard_data)
                    .service(module::campaign::get_campaign_dashboard_productive_operator)
                    .service(module::campaign::get_campaign_by_id)
                    .service(module::campaign::get_campaign_by_slug),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
