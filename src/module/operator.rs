use crate::{dto::*, model::*};

use actix_web::{get, post, delete, web, HttpResponse, Responder};

use sqlx::MySqlPool;
use tracing::error;
use uuid::Uuid;

#[post("/add")]
pub async fn add_operator(
    db: web::Data<MySqlPool>,
    payload: web::Json<AddOperatorRequest>,
) -> impl Responder {
    let query = "
    INSERT INTO operator
    (uuid, channel, identity, schedule, name, nickname, status, created_at, updated_at)
    VALUES (?, ?, ?, ?, ?, ?, ?, NOW(), NOW())
";

    let schedule_string = payload.schedule.join(",");
    let result = sqlx::query(query)
        .bind(Uuid::new_v4().to_string())
        .bind(&payload.channel)
        .bind(&payload.identity)
        .bind(&schedule_string)
        .bind(&payload.name)
        .bind(&payload.nickname)
        .bind(&payload.status)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(BasicResponse {
            success: true,
            message: "Successfuly add the operator".to_string(),
        }),
        Err(e) => {
            error!("{}", e);

            HttpResponse::Unauthorized().json(BasicResponse {
                success: false,
                message: "Something wrong!".to_string(),
            })
        }
    }
}

#[get("/list")]
pub async fn list_operator(db: web::Data<MySqlPool>) -> impl Responder {
    let query = "SELECT * FROM operator";

    let result = sqlx::query_as::<_, Operator>(query)
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(operators) => HttpResponse::Ok().json(DataResponse {
            success: true,
            message: "Successfully fetched operators".to_string(),
            data: Some(serde_json::json!(operators)),
        }),
        Err(e) => {
            error!("{}", e);

            HttpResponse::InternalServerError().json(BasicResponse {
                success: false,
                message: "Something went wrong".to_string(),
            })
        }
    }
}

#[get("/list-option")]
pub async fn list_option(db: web::Data<MySqlPool>) -> impl Responder {
    let query = "SELECT
        uuid AS value,
        CONCAT(
            CASE channel
            WHEN 1 THEN 'Whatsapp'
            WHEN 2 THEN 'Instagram'
            WHEN 3 THEN 'Email'
            ELSE 'Unknown'
            END,
            ' - ', identity, ' - ', name
        ) AS title
        FROM operator";

    let result = sqlx::query_as::<_, ListOption>(query)
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(operators) => HttpResponse::Ok().json(DataResponse {
            success: true,
            message: "Successfully fetched operators".to_string(),
            data: Some(serde_json::json!(operators)),
        }),
        Err(e) => {
            error!("{}", e);

            HttpResponse::InternalServerError().json(BasicResponse {
                success: false,
                message: "Something went wrong".to_string(),
            })
        }
    }
}

#[delete("/delete/{uuid}")]
pub async fn delete_operator(
    db: web::Data<MySqlPool>,
    uuid: web::Path<String>,
) -> impl Responder {
    let result = sqlx::query("DELETE FROM operator WHERE uuid = ?")
        .bind(uuid.into_inner())
        .execute(db.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(BasicResponse {
            success: true,
            message: "Operator deleted successfully".to_string(),
        }),
        Ok(_) => HttpResponse::NotFound().json(BasicResponse {
            success: false,
            message: "Operator not found".to_string(),
        }),
        Err(e) => {
            error!("Failed to delete operator: {}", e);
            HttpResponse::InternalServerError().json(BasicResponse {
                success: false,
                message: "Failed to delete operator".to_string(),
            })
        }
    }
}