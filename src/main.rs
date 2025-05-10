use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //Redirect all `log`s events to our subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let port = &configuration.application.port;
    let host = &configuration.application.host;
    let listener = TcpListener::bind(format!("{host}:{port}"));
    let connection_pool =
        PgPoolOptions::new().connect_lazy_with(configuration.database.connection_options());
    run(listener.expect("Failed to bind"), connection_pool)?.await
}
