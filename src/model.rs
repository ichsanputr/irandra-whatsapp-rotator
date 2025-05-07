use chrono::NaiveDateTime;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub uuid: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Operator {
    pub uuid: String,
    pub channel: i32,
    pub identity: String,
    pub schedule: Option<String>,
    pub name: String,
    pub nickname: String,
    pub status: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Campaign {
    pub uuid: String,
    pub slug: String,
    pub message: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}