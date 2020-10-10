use serde::{Deserialize, Serialize};
use serde_json::{error::Error as JsonError, to_string as to_json};
use std::sync::MutexGuard;
use tide::{Error, Request, Response, StatusCode};

use crate::models::item::Item;
use crate::models::list::List;
use crate::models::state::State;

pub const ITEM_NOT_FOUND: &str = "Item not found";
pub const SAVING_ITEM_FAILED: &str = "Saving item failed";
const JSON_CONVERSION_FAILED: &str = "JSON conversion failed";

pub type TideResult = tide::Result<Response>;

#[derive(Deserialize)]
pub struct Input {
    pub active: bool,
    pub name: String,
    pub content: String,
}

#[derive(Serialize)]
struct Output {
    id: u64,
    active: bool,
    name: String,
    content: String,
}

impl Output {
    fn new(id: u64, name: &str, content: &str, active: bool) -> Self {
        Self {
            id,
            active,
            name: name.to_owned(),
            content: content.to_owned(),
        }
    }
    fn from(item: &Item) -> Self {
        Output::new(item.id, &item.name, &item.content, item.active)
    }
    fn json(&self) -> Result<String, JsonError> {
        to_json(self)
    }
}

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
    if let Ok(json) = Output::from(item).json() {
        json_response(&json)
    } else {
        error(StatusCode::InternalServerError, JSON_CONVERSION_FAILED)
    }
}

pub fn json_list(list: &List) -> TideResult {
    let output: Vec<Output> = list.all().iter().map(|item| Output::from(item)).collect();
    if let Ok(json) = to_json(&output) {
        json_response(&json)
    } else {
        error(StatusCode::InternalServerError, JSON_CONVERSION_FAILED)
    }
}

pub fn error(status: StatusCode, content: &'static str) -> TideResult {
    Err(Error::from_str(status, content))
}
