
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub userid: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimsGoogle {
    pub aud: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimsX {
    pub firebase: ClaimsXData
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimsXData {
    pub identities: HashMap<String, Vec<String>>,
}

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
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginGoogleRequest {
    pub email: String,
    pub name: String,
    pub picurl: String,
    pub oauthtoken: String,
    pub idtoken: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginXRequest {
    pub name: String,
    pub iduser: String,
    pub idtoken: String,
}

#[derive(Serialize, Deserialize)]
pub struct TurnstileRequest {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct TurnstileResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: String,
    pub email: String,
    pub pic_url: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTripayPayment {
    pub day: i32,
    pub amount: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackTripay {
    pub reference: String,
    pub merchant_ref: String,
    pub payment_method: String,
    pub payment_method_code: String,
    pub total_amount: i32,
    pub fee_merchant: i32,
    pub fee_customer: i32,
    pub total_fee: i32,
    pub amount_received: i32,
    pub is_closed_payment: i32,
    pub status: String,
    pub paid_at: i64,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfirmPaypal {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadMediaRequest {
    pub file: String,
    pub file_name: String,
    pub mime_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddActivityRequest {
    pub kind: String,
    pub resource: String,
}
#[derive(Serialize, Deserialize)]
pub struct AddSentence {
    pub kind_category: i32,
    pub length_category: i32,
    pub languange: i32,
    pub sentence: String,
    pub fill: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AddResult {
    pub sentence_id: Option<String>,
    pub room_id: Option<String>,
    pub time: i8,
    pub wrong: i8,
}

#[derive(Serialize, Deserialize)]
pub struct ReportSentence {
    pub sentence_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyWeb3 {
    pub provider: String,
    pub public_key: String,
    pub signature: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDetail {
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDetailResponse {
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub image: String,
    pub balance: f64,
}
#[derive(serde::Deserialize)]
pub struct PaypalQuery {
    pub order_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddCraftfmeQuestion {
    pub category: i32,
    pub languange: i32,
    pub text: String,
}