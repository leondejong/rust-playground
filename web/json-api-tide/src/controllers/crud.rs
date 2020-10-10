use tide::{Request, StatusCode};

use super::utils::*;
use crate::models::list::List;
use crate::models::state::State;

pub async fn all(request: Request<State<List>>) -> TideResult {
    let list = get_list(&request);
    json_list(&list)
}

pub async fn one(request: Request<State<List>>) -> TideResult {
    let id: u64 = request.param("id")?;
    let list = get_list(&request);
    if let Some(item) = list.get(id) {
        json_item(&item)
    } else {
        error(StatusCode::NotFound, ITEM_NOT_FOUND)
    }
}

pub async fn add(mut request: Request<State<List>>) -> TideResult {
    let input: Input = request.body_json().await?;
    let mut list = get_list(&request);
    if let Some(item) = list.add(&input.name, &input.content, input.active) {
        json_item(&item)
    } else {
        error(StatusCode::InternalServerError, SAVING_ITEM_FAILED)
    }
}

pub async fn update(mut request: Request<State<List>>) -> TideResult {
    let id: u64 = request.param("id")?;
    let input: Input = request.body_json().await?;
    let mut list = get_list(&request);
    if let Some(item) = list.update(id, &input.name, &input.content, input.active) {
        json_item(&item)
    } else {
        error(StatusCode::NotFound, ITEM_NOT_FOUND)
    }
}

pub async fn remove(request: Request<State<List>>) -> TideResult {
    let id: u64 = request.param("id")?;
    let mut list = get_list(&request);
    if let Some(item) = list.remove(id) {
        json_item(&item)
    } else {
        error(StatusCode::NotFound, ITEM_NOT_FOUND)
    }
}

pub async fn info(mut request: Request<State<List>>) -> TideResult {
    let method = request.method();
    let url = request.url().clone();
    let body: String = request.body_string().await?;
    let json = format!(
        "{{\"method\": \"{}\", \"url\": \"{}\", \"body\": \"{}\"}}",
        method,
        url,
        body.replace("\"", "\\\"")
    );
    json_response(&json)
}
