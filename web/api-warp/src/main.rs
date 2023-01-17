use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{body::json, delete, get, path, path::end, post, put, serve, Filter};

mod controllers;
mod models;

use controllers::{get as one, index, insert, remove, update, with};
use models::List;

// http http://127.0.0.1:9277
// http http://127.0.0.1:9277/1
// http post http://127.0.0.1:9277 name=name content=content active:=true
// http put http://127.0.0.1:9277/1 name=name content=content active:=true
// http delete http://127.0.0.1:9277/1

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(List::new()));

    let index = end().and(get()).and(with(state.clone())).and_then(index);

    let get_item = path!(u32).and(get()).and(with(state.clone())).and_then(one);

    let insert_item = end()
        .and(post())
        .and(json())
        .and(with(state.clone()))
        .and_then(insert);

    let update_item = path!(u32)
        .and(put())
        .and(json())
        .and(with(state.clone()))
        .and_then(update);

    let remove_item = path!(u32)
        .and(delete())
        .and(with(state.clone()))
        .and_then(remove);

    let routes = index
        .or(get_item)
        .or(insert_item)
        .or(update_item)
        .or(remove_item);

    serve(routes).run(([127, 0, 0, 1], 9277)).await;
}
