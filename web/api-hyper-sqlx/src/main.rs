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

// http http://127.0.0.1:4973
// http http://127.0.0.1:4973/1
// http post http://127.0.0.1:4973 name=name content=content active:=true
// http put http://127.0.0.1:4973/1 name=name content=content active:=true
// http delete http://127.0.0.1:4973/1

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let address = ([127, 0, 0, 1], 4973).into();
    let service = router().await.unwrap();
    let server = Server::bind(&address).serve(service);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    graceful.await?;

    Ok(())
}
