use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{body::json, delete, get, path, path::end, post, put, serve, Filter};

mod controllers;
mod models;

use controllers::{add, all, one, remove, update, with};
use models::List;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(List::new()));

    let index = end().and(get()).and(with(state.clone())).and_then(all);

    let show_item = path!("item" / u32)
        .and(get())
        .and(with(state.clone()))
        .and_then(one);

    let add_item = path("item")
        .and(post())
        .and(json())
        .and(with(state.clone()))
        .and_then(add);

    let update_item = path!("item" / u32)
        .and(put())
        .and(json())
        .and(with(state.clone()))
        .and_then(update);

    let remove_item = path!("item" / u32)
        .and(delete())
        .and(with(state.clone()))
        .and_then(remove);

    let routes = index
        .or(show_item)
        .or(add_item)
        .or(update_item)
        .or(remove_item);

    serve(routes).run(([127, 0, 0, 1], 9277)).await;
}
