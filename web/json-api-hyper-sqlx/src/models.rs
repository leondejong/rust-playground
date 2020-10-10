use anyhow::Result as AnyResult;
use chrono::prelude::*;
use dotenv::dotenv;
use sqlx::{postgres::PgPool, Done, Error, FromRow};
use std::env::{self, VarError};

#[derive(Debug, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub active: bool,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

pub fn database_url() -> AnyResult<String, VarError> {
    dotenv().ok();

    env::var("DATABASE_URL")
}

pub async fn database_pool() -> AnyResult<PgPool, Error> {
    let database_url = database_url().expect("DATABASE_URL not set");

    Ok(PgPool::connect(&database_url).await?)
}

pub async fn all(pool: &PgPool) -> AnyResult<Vec<Item>> {
    let items = sqlx::query_as!(
        Item,
        r#"
SELECT * FROM items
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(items)
}

pub async fn one(pool: &PgPool, id: i32) -> AnyResult<Item> {
    let items = sqlx::query_as!(
        Item,
        r#"
SELECT * FROM items WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(items)
}

pub async fn add(pool: &PgPool, name: &str, content: &str, active: bool) -> AnyResult<i32> {
    let record = sqlx::query!(
        r#"
INSERT INTO items ( name, content, active )
VALUES ( $1, $2, $3 )
RETURNING id
        "#,
        name,
        content,
        active,
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    name: &str,
    content: &str,
    active: bool,
) -> AnyResult<u64> {
    let affected = sqlx::query!(
        r#"
UPDATE items
SET name = $1, content = $2, active = $3, updated = $4
WHERE id = $5
        "#,
        name,
        content,
        active,
        Utc::now(),
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(affected)
}

pub async fn remove(pool: &PgPool, id: i32) -> AnyResult<u64> {
    let affected = sqlx::query!(
        r#"
DELETE FROM items
WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(affected)
}
