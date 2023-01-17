use hyper::{body, Body, Request, Response, StatusCode};
use routerify::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_slice, to_string};
use std::convert::Infallible;

pub type HyperResult = Result<Response<Body>, Infallible>;

#[derive(Deserialize)]
pub struct Input {
    pub name: String,
    pub content: String,
    pub active: bool,
}

fn status(code: StatusCode) -> HyperResult {
    let mut response = Response::default();
    *response.status_mut() = code;
    Ok(response)
}

pub fn ok() -> HyperResult {
    status(StatusCode::OK)
}

pub fn bad_request() -> HyperResult {
    status(StatusCode::BAD_REQUEST)
}

pub fn not_found() -> HyperResult {
    status(StatusCode::NOT_FOUND)
}

pub fn internal_server_error() -> HyperResult {
    status(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn to_struct<T>(request: Request<Body>) -> Result<T, &'static str>
where
    T: DeserializeOwned,
{
    if let Ok(bytes) = body::to_bytes(request.into_body()).await {
        if let Ok(structure) = from_slice::<T>(&bytes[..]) {
            return Ok(structure);
        }
    }
    Err("conversion to structure failed")
}

pub fn to_json<T>(structure: T) -> Result<Response<Body>, &'static str>
where
    T: Serialize,
{
    if let Ok(json) = to_string(&structure) {
        return Ok(Response::new(Body::from(json)));
    }
    Err("conversion to JSON failed")
}

pub fn get_id(request: &Request<Body>) -> Option<i32> {
    request.param("id")?.parse::<i32>().ok()
}
