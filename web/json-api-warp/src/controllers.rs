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

pub async fn all(state: State) -> Result<impl Reply, Infallible> {
    let list = state.lock().await;
    Ok(json(list.all()))
}

pub async fn one(id: u32, state: State) -> Result<Response, Infallible> {
    let list = state.lock().await;
    if let Some(item) = list.one(id) {
        Ok(json(item).into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}

pub async fn add(input: Input, state: State) -> Result<Response, Infallible> {
    let mut list = state.lock().await;
    list.add(&input.name, &input.content, input.active);
    Ok(StatusCode::OK.into_response())
}

pub async fn update(id: u32, input: Input, state: State) -> Result<Response, Infallible> {
    let mut list = state.lock().await;
    if let Some(_item) = list.one(id) {
        list.update(id, &input.name, &input.content, input.active);
        Ok(StatusCode::OK.into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}

pub async fn remove(id: u32, state: State) -> Result<Response, Infallible> {
    let mut list = state.lock().await;
    if let Some(_item) = list.one(id) {
        list.remove(id);
        Ok(StatusCode::OK.into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}
