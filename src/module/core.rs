use crate::{dto::*, model::*};

use actix_web::{get, web, Responder, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Duration, Utc};
// use serde::{Deserialize, Serialize};

use sqlx::MySqlPool;

#[get("/")]
pub async fn main(
    db: web::Data<MySqlPool>,
    slug: web::Path<String>
) -> impl Responder {
    let query = "SELECT * FROM campaign WHERE slug = ?";

    let result = sqlx::query_as::<_, User>(query)
        .bind(slug.as_ref())
        .fetch_optional(db.get_ref())
        .await;

    if let Ok(Some(user)) = result {
        let expiration = Utc::now() + Duration::hours(24);
        let claims = Claims {
            userid: user.uuid,
            exp: expiration.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("catsentence_18219".as_ref()),
        )
        .unwrap();
        
        HttpResponse::Ok().json(DataResponse {
            success: true,
            message: "Login successful".to_string(),
            data: Some(serde_json::json!({ "token": token })),
        })
    } else {
        HttpResponse::Unauthorized().json(BasicResponse {
            success: false,
            message: "Wrong username & password!".to_string(),
        })
    }
}