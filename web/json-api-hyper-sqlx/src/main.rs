use hyper::Server;
use std::error::Error;

mod controllers;
mod models;
mod utils;

use controllers::router;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let address = ([127, 0, 0, 1], 4973).into();
    let service = router().await.unwrap();
    let server = Server::bind(&address).serve(service);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    graceful.await?;

    Ok(())
}
