use grok::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("0.0.0.0:8080")
        .expect("Failed to bind.");
    run(listener)?.await
}
