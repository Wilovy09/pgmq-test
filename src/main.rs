use actix_cors::Cors;
use actix_web::{App, HttpServer, web::Data};
use sqlx::Pool;

use crate::{
    config::Config,
    helpers::{prometheus_logs::PROMETHEUS, verify_pgmq::verify_pgmq_extension},
};

mod config;
mod errors;
mod helpers;
mod mailer;
mod middlewares;
mod users;

pub struct AppState {
    pub db_pool: Pool<sqlx::Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let client = Pool::<sqlx::Postgres>::connect(&Config::from_env().database_url)
        .await
        .expect("Failed to connect to the database");

    verify_pgmq_extension(&client).await;

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(Data::new(AppState {
                db_pool: client.clone(),
            }))
            .wrap(cors)
            .wrap(PROMETHEUS.clone())
            .configure(users::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
