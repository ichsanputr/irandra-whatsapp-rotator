use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

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
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct AddOperatorRequest {
    #[validate(range(min = 1))]
    pub channel: i32,

    #[validate(length(min = 1, message = "identity cannot be empty"))]
    pub identity: String,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1))]
    pub nickname: String,

    pub schedule: Vec<String>,
    pub status: i8,
}

pub type UpdateOperatorRequest = AddOperatorRequest;

// Campaign
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct OperatorAssignment {
    pub id: String,
    pub grade: u8,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct CampaignSlug {
    pub uuid: String,
    pub operator_id: String,
    pub identity: String,
    pub grade: i32,
    pub handle: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCampaignRequest {
    pub name: String,
    pub message: String,
    pub slug: String,
    pub operators: Vec<OperatorAssignment>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct CampaignOperator {
    pub id: String,
    pub grade: i32,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ReportVisitor {
    pub uuid: String,
    pub operator_name: String,
    pub device: String,
    pub maps: String,
    pub ip_address: String,
    pub location: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ReportChart {
    pub operator_name: String,
    pub total: i32,
    pub report_date: NaiveDate,
}

#[derive(Deserialize)]
pub struct ChartQuery {
    pub start_date: String,
    pub end_date: String,
}
