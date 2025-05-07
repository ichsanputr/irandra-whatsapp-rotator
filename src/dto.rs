
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Core
#[derive(Serialize, Deserialize)]
pub struct BasicResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub userid: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ListOption {
    pub title: String,
    pub value: String,
}

// Auth
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// Operator
#[derive(Serialize, Deserialize, Debug)]
pub struct AddOperatorRequest {
    pub channel: i32,
    pub identity: String,
    pub schedule: Vec<String>,
    pub name: String,
    pub nickname: String,
    pub status: i8,
}

// Campaign
#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorAssignment {
    pub id: String,
    pub grade: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCampaignRequest {
    pub name: String,
    pub message: String,
    pub slug: String,
    pub operators: Vec<OperatorAssignment>,
}