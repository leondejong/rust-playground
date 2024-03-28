use std::convert::Infallible;

use warp::{
    http::StatusCode,
    reply::{json, Response},
    Filter, Reply,
};

use crate::models::{Input, State};

pub fn with(state: State) -> impl Filter<Extract = (State,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub async fn index(state: State) -> Result<impl Reply, Infallible> {
    let list = state.lock().await;
    Ok(json(&list.index()))
}

pub async fn get(id: u32, state: State) -> Result<Response, Infallible> {
    let list = state.lock().await;
    if let Some(item) = list.get(id) {
        Ok(json(&item).into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}

pub async fn insert(input: Input, state: State) -> Result<Response, Infallible> {
    let mut list = state.lock().await;
    if let Some(item) = list.insert(&input.name, &input.content, input.active) {
        Ok(json(&item).into_response())
    } else {
        Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
    }
}

pub async fn update(id: u32, input: Input, state: State) -> Result<Response, Infallible> {
    let mut list = state.lock().await;
    if let Some(item) = list.get(id) {
        let item = item.clone();
        list.update(id, &input.name, &input.content, input.active);
        Ok(json(&item).into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}

pub async fn remove(id: u32, state: State) -> Result<Response, Infallible> {
    let mut list = state.lock().await;
    if let Some(item) = list.get(id) {
        let item = item.clone();
        list.remove(id);
        Ok(json(&item).into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}
