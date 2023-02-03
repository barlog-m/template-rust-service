use std::{env, net::SocketAddr};
use tracing_subscriber::filter::EnvFilter;

use {{crate_name}}::app;

#[tokio::main]
async fn main() {
    tracer();

    let app = app();

    let port = port();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn tracer() {
    let filter = match EnvFilter::try_from_default_env() {
        Ok(f) => f,
        _ => {
            env::set_var(
                "RUST_LOG",
                "axum=debug,tower_http=debug,hyper=debug,tokio=trace,debug",
            );
            EnvFilter::from_default_env()
        }
    };

    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn port() -> u16 {
    let default_port = 8080;

    match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or(default_port),
        Err(_) => default_port,
    }
}
