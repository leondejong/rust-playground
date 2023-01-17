use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::models::{Input, State as Store};

pub async fn list(State(state): State<Store>) -> Result<impl IntoResponse, StatusCode> {
    let items = state
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .index();

    Ok((StatusCode::OK, Json(items)))
}

pub async fn item(
    Path(id): Path<u32>,
    State(state): State<Store>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = state
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .get(id)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn add(
    State(state): State<Store>,
    Json(input): Json<Input>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = state
        .write()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .insert(&input.name, &input.content, input.active)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Store>,
    Json(input): Json<Input>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = state
        .write()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .update(id, &input.name, &input.content, input.active)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn remove(
    Path(id): Path<u32>,
    State(state): State<Store>,
) -> Result<impl IntoResponse, StatusCode> {
    let item = state
        .write()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .remove(id)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(item)))
}
