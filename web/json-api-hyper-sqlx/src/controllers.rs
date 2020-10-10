use hyper::{Body, Request};
use routerify::{prelude::*, Error as RouteError, Router, RouterService};
use sqlx::postgres::PgPool;
use std::convert::Infallible;

use crate::models::{add, all, database_pool, one, remove, update};
use crate::utils::*;

async fn all_items(request: Request<Body>) -> HyperResult {
    let pool = match request.data::<PgPool>() {
        Some(pool) => pool,
        _ => return internal_server_error(),
    };

    let list = match all(&pool).await {
        Ok(list) => list.iter().map(Info::from).collect(),
        _ => return not_found(),
    };

    to_json::<Vec<Info>>(list).or_else(|_| internal_server_error())
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

    to_json::<Info>(Info::from(&item)).or_else(|_| internal_server_error())
}

async fn add_item(request: Request<Body>) -> HyperResult {
    let pool = match request.data::<PgPool>() {
        Some(pool) => pool.clone(),
        _ => return internal_server_error(),
    };

    let input = match to_struct::<Content>(request).await {
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

    let input = match to_struct::<Content>(request).await {
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

pub async fn router() -> Result<RouterService<Body, Infallible>, RouteError> {
    let pool = database_pool().await.unwrap();
    let router = Router::builder()
        .data(pool)
        .get("/", all_items)
        .get("/item/:id", one_item)
        .post("/item", add_item)
        .put("/item/:id", update_item)
        .delete("/item/:id", remove_item)
        .build()
        .unwrap();
    RouterService::new(router)
}
