use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AuthorModel {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BookModel {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
}
