use axum::{routing::get, Router};
use std::{net::SocketAddr, sync::Arc};

use crate::controllers::{add, item, list, remove, update};
use crate::models::State as Store;

mod controllers;
mod models;

// http http://127.0.0.1:2986
// http http://127.0.0.1:2986/1
// http post http://127.0.0.1:2986 name=name content=content active:=true
// http put http://127.0.0.1:2986/1 name=name content=content active:=true
// http delete http://127.0.0.1:2986/1

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 2986));

    let state = Store::default();

    let app = Router::new()
        .route("/", get(list).post(add))
        .route("/:id", get(item).put(update).delete(remove))
        .with_state(Arc::clone(&state));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
