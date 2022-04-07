use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;

mod handlers;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(handlers::health_check))
            .route("/event", web::post().to(handlers::event))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
