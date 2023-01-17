use serde_json::to_string;
use std::sync::MutexGuard;
use tide::{Error, Request, Response, StatusCode};

use crate::models::{Item, List, State};

pub const ITEM_NOT_FOUND: &str = "Item not found";
pub const SAVING_ITEM_FAILED: &str = "Saving item failed";
pub const JSON_CONVERSION_FAILED: &str = "JSON conversion failed";

pub type TideResult = tide::Result<Response>;

pub fn get_list<'a, T: Default>(request: &'a Request<State<T>>) -> MutexGuard<'a, T> {
    request.state().data.lock().unwrap()
}

pub fn json_response(content: &str) -> TideResult {
    Ok(Response::builder(StatusCode::Ok)
        .header("content-type", "application/json")
        .body(content)
        .build())
}

pub fn json_item(item: &Item) -> TideResult {
    if let Ok(json) = to_string(item) {
        json_response(&json)
    } else {
        error(StatusCode::InternalServerError, JSON_CONVERSION_FAILED)
    }
}

pub fn json_list(list: &List) -> TideResult {
    let items = list.index();
    if let Ok(json) = to_string(&items) {
        json_response(&json)
    } else {
        error(StatusCode::InternalServerError, JSON_CONVERSION_FAILED)
    }
}

pub fn error(status: StatusCode, content: &'static str) -> TideResult {
    Err(Error::from_str(status, content))
}
