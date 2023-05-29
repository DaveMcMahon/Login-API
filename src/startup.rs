use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::health_check::health_check;
use crate::routes::logins::login;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/login", web::post().to(login))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
