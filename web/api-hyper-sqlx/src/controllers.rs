use hyper::{Body, Request};
use routerify::{prelude::*, Router, RouterService};
use sqlx::postgres::PgPool;
use std::convert::Infallible;
use std::error::Error;

use crate::models::{add, all, database_pool, one, remove, update, Item};
use crate::utils::*;

async fn all_items(request: Request<Body>) -> HyperResult {
    let pool = match request.data::<PgPool>() {
        Some(pool) => pool,
        _ => return internal_server_error(),
    };

    let list = match all(&pool).await {
        Ok(list) => list,
        _ => return not_found(),
    };

    to_json::<Vec<Item>>(list).or_else(|_| internal_server_error())
}

async fn one_item(request: Request<Body>) -> HyperResult {
    let pool = match request.data::<PgPool>() {
        Some(pool) => pool,
        _ => return internal_server_error(),
    };

    let id = match get_id(&request) {
        Some(id) => id,
        _ => return bad_request(),
    };

    let item = match one(&pool, id).await {
        Ok(item) => item,
        _ => return not_found(),
    };

    to_json::<Item>(item).or_else(|_| internal_server_error())
}

async fn add_item(request: Request<Body>) -> HyperResult {
    let pool = match request.data::<PgPool>() {
        Some(pool) => pool.clone(),
        _ => return internal_server_error(),
    };

    let input = match to_struct::<Input>(request).await {
        Ok(input) => input,
        _ => return bad_request(),
    };

    match add(&pool, &input.name, &input.content, input.active).await {
        Ok(_id) => ok(),
        _ => internal_server_error(),
    }
}

async fn update_item(request: Request<Body>) -> HyperResult {
    let pool = match request.data::<PgPool>() {
        Some(pool) => pool.clone(),
        _ => return internal_server_error(),
    };

    let id = match get_id(&request) {
        Some(id) => id,
        _ => return bad_request(),
    };

    let input = match to_struct::<Input>(request).await {
        Ok(input) => input,
        _ => return bad_request(),
    };

    match update(&pool, id, &input.name, &input.content, input.active).await {
        Ok(affected) if affected > 0 => return ok(),
        _ => not_found(),
    }
}

async fn remove_item(request: Request<Body>) -> HyperResult {
    let pool = match request.data::<PgPool>() {
        Some(pool) => pool,
        _ => return internal_server_error(),
    };

    let id = match get_id(&request) {
        Some(id) => id,
        _ => return bad_request(),
    };

    match remove(&pool, id).await {
        Ok(affected) if affected > 0 => return ok(),
        _ => not_found(),
    }
}

pub async fn router(
) -> Result<RouterService<Body, Infallible>, Box<(dyn Error + Send + Sync + 'static)>> {
    let pool = database_pool().await.unwrap();
    let router = Router::builder()
        .data(pool)
        .get("/", all_items)
        .get("/:id", one_item)
        .post("/", add_item)
        .put("/:id", update_item)
        .delete("/:id", remove_item)
        .build()
        .unwrap();
    RouterService::new(router)
}
