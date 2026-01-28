use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web::Data};
use actix_web_prometheus::PrometheusMetricsBuilder;
use sqlx::Pool;

use crate::config::Config;

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

    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

    // Verify if pgmq extension is available
    // If not, create it
    // You can see https://github.com/pgmq/pgmq/blob/main/INSTALLATION.md
    match sqlx::query!("SELECT * FROM pg_available_extensions WHERE NAME = 'pgmq';")
        .fetch_one(&client)
        .await
    {
        Ok(record) => {
            println!("Verified pgmq extension {:?}", record);
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
    match sqlx::query!("CREATE EXTENSION IF NOT EXISTS pgmq;")
        .execute(&client)
        .await
    {
        Ok(_) => {
            println!("pgmq extension created or already exists");
        }
        Err(e) => {
            eprintln!("Failed to create pgmq extension: {e}");
        }
    }

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(Data::new(AppState {
                db_pool: client.clone(),
            }))
            .wrap(cors)
            .wrap(prometheus.clone())
            .configure(users::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
