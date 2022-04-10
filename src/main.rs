use grok::run;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::read_config().unwrap();
    let bind = config.general.bind;
    let port = config.general.port;
    let listener = std::net::TcpListener::bind(format!("{}:{}", bind, port))
        .expect("Failed to bind.");
    run(listener)?.await
}
