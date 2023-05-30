use login_api::configuration::get_configuration;
use login_api::startup::run;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use std::process;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let database_config = get_configuration().expect("Failed to read config from config.yaml");
    let connection = PgConnection::connect(&database_config.database.get_connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind("127.0.0.1:8081").unwrap_or_else(|err| {
        println!("Error binding to address {}", err);
        process::exit(1);
    });

    run(listener, connection)?.await
}
