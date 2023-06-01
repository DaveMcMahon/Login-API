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

    let http_server_string = format!("127.0.0.1:{}", &database_config.app_port.to_string());
    let listener = TcpListener::bind(http_server_string)?;

    run(listener, connection_pool)?.await
}
