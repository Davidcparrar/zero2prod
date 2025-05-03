use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let port = configuration.application.port;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"));
    run(listener.expect("Failed to bind"))?.await
}
