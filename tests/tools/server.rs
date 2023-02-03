use std::future::Future;
use std::net::SocketAddr;
use std::env;

use crate::tools::TestError;

use {{crate_name}}::app;

pub async fn with_server<T, R>(f: T) -> Result<(), TestError>
where
    T: Fn(SocketAddr) -> R,
    R: Future<Output = Result<(), TestError>>,
{
    tracer();
    let app = app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 0));

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
    let server_addr = server.local_addr();

    tokio::spawn(server);

    tracing::debug!("listening on {}", server_addr);

    f(server_addr).await?;
    Ok(())
}

fn tracer() {
    env::set_var("RUST_LOG", "axum=debug,tower_http=debug,tokio=trace,hyper=warn,debug");
    tracing_subscriber::fmt::init();
}
