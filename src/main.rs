use std::net::TcpListener;
use std::process;

use login_api::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap_or_else(|err| {
        println!("Error binding to address {}", err);
        process::exit(1);
    });
    run(listener)?.await
}
