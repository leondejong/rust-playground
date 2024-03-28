use tide::{Request, StatusCode};

use crate::models::{Input, List, State};
use crate::utils::*;

pub async fn all(request: Request<State<List>>) -> TideResult {
    let list = get_list(&request);
    let items = list.index();
    json_response(&items)
}

pub async fn one(request: Request<State<List>>) -> TideResult {
    let id = request.param("id")?.parse::<u32>()?;
    let list = get_list(&request);
    if let Some(item) = list.get(id) {
        json_response(&item)
    } else {
        error(StatusCode::NotFound, ITEM_NOT_FOUND)
    }
}

pub async fn add(mut request: Request<State<List>>) -> TideResult {
    let input: Input = request.body_json().await?;
    let mut list = get_list(&request);
    if let Some(item) = list.insert(&input.name, &input.content, input.active) {
        json_response(&item)
    } else {
        error(StatusCode::InternalServerError, SAVING_ITEM_FAILED)
    }
}

pub async fn update(mut request: Request<State<List>>) -> TideResult {
    let id = request.param("id")?.parse::<u32>()?;
    let input: Input = request.body_json().await?;
    let mut list = get_list(&request);
    if let Some(item) = list.update(id, &input.name, &input.content, input.active) {
        json_response(&item)
    } else {
        error(StatusCode::NotFound, ITEM_NOT_FOUND)
    }
}

pub async fn remove(request: Request<State<List>>) -> TideResult {
    let id = request.param("id")?.parse::<u32>()?;
    let mut list = get_list(&request);
    if let Some(item) = list.remove(id) {
        json_response(&item)
    } else {
        error(StatusCode::NotFound, ITEM_NOT_FOUND)
    }
}
