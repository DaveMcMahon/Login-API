use login_api::configuration::get_configuration;
use login_api::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let database_config = get_configuration().expect("Failed to read config from config.yaml");
    let connection_pool = PgPool::connect(&database_config.database.get_connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind("127.0.0.1:8081")?;

    run(listener, connection_pool)?.await
}
