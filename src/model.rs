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
pub struct Payments {
    pub uuid: String,
    pub user_id: String,
    pub reference: String,
    pub payment_name: String,
    pub checkout_url: String,
    pub status: Option<String>,
    pub expired_time: Option<String>,
    pub day: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Fillme {
    pub uuid: String,
    pub sentence: String,
    pub fill: String,
    pub length: Option<i8>,
    pub category: Option<i8>,
    pub user_id: Option<String>,
    pub reported: Option<i8>,
    pub languange: Option<i8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct FillmeLeaderboard {
    pub played: Option<i32>,
    pub name: Option<String>,
    pub total_time: Option<i32>,
    pub total_point: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct FillmeRoom {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub lang: String,
    pub played: Option<i32>,
    pub is_private: Option<i8>,
    pub room_id: Option<String>,
    pub sentences: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct FillmeRoomCleaned {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub lang: String,
    pub played: Option<i32>,
    pub is_private: Option<i8>,
    pub room_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct FillmeRoomResponse {
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub room: Option<FillmeRoomCleaned>,
    pub sentences: Option<Vec<Fillme>>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct FillmePlayed {
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub fillme: Fillme, 
    pub played: Option<i32>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Activity {
    pub user_id: String, 
    pub kind: String,
    pub resource: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Results {
    pub room_id: Option<String>,
    pub sentence_id: Option<String>,
    pub room_name: Option<String>,
    pub source: String,
    pub time: Option<i8>,
    pub wrong: Option<i8>,
    pub total_point: Option<i8>,
    pub user_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Craftme {
    pub uuid: String,
    pub text: String,
    pub category: Option<i8>,
    pub user_id: Option<String>,
    pub reported: Option<i8>,
    pub languange: Option<i8>,
    pub is_room: Option<i8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct CraftmeList {
    pub uuid: String,
    pub text: String,
    pub category: Option<i8>,
    pub user_id: Option<String>,
    pub reported: Option<i8>,
    pub languange: Option<i8>,
    pub is_room: Option<i8>,
    pub played: Option<i8>,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}