use crate::configuration::*;
use crate::startup::run;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct AppData {
    pub address_string: String,
    pub connection: PgPool,
}

async fn setup_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.get_connection_string_without_db())
        .await
        .expect("Couldn't connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed tom create database");

    let connection_pool = PgPool::connect(&config.get_connection_string())
        .await
        .expect("Failed to connect to postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to update database schema");

    connection_pool
}

pub async fn spawn_app() -> AppData {
    let mut config = get_configuration().expect("Couldn't read config from config.yaml");

    config.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = setup_database(&config.database).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let address = format!("http://127.0.0.1:{}", port);

    let _ = tokio::spawn(server);

    AppData {
        address_string: address,
        connection: connection_pool,
    }
}
