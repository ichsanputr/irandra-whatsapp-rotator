use crate::{dto::*, model::*};

use actix_web::{delete, get, post, web, HttpResponse, Responder};

use sqlx::MySqlPool;
use tracing::error;
use uuid::Uuid;

#[post("/add")]
pub async fn add_campaign(
    db: web::Data<MySqlPool>,
    payload: web::Json<AddCampaignRequest>,
) -> impl Responder {
    let campaign_id = Uuid::new_v4().to_string();

    let mut tx = match db.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json(BasicResponse {
                success: false,
                message: "Failed to start transaction".to_string(),
            });
        }
    };

    let result = sqlx::query(
        "
        INSERT INTO campaign
        (uuid, name, message, slug, created_at, updated_at)
        VALUES (?, ?, ?, ?, NOW(), NOW())
    ",
    )
    .bind(&campaign_id)
    .bind(&payload.name)
    .bind(&payload.message)
    .bind(&payload.slug)
    .execute(&mut *tx)
    .await;

    if let Err(e) = result {
        error!("Failed to insert campaign: {}", e);
        tx.rollback().await.ok();
        return HttpResponse::InternalServerError().json(BasicResponse {
            success: false,
            message: "Failed to insert campaign".to_string(),
        });
    }

    for op in &payload.operators {
        let result = sqlx::query(
            "
            INSERT INTO campaign_operator
            (uuid, campaign_id, operator_id, grade, created_at, updated_at)
            VALUES (?, ?, ?, ?, NOW(), NOW())
        ",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&campaign_id)
        .bind(&op.id)
        .bind(op.grade)
        .execute(&mut *tx)
        .await;

        if let Err(e) = result {
            error!("Failed to insert campaign_operator: {}", e);
            tx.rollback().await.ok();
            return HttpResponse::InternalServerError().json(BasicResponse {
                success: false,
                message: "Failed to insert campaign_operator".to_string(),
            });
        }
    }

    if let Err(e) = tx.commit().await {
        error!("Failed to commit transaction: {}", e);
        return HttpResponse::InternalServerError().json(BasicResponse {
            success: false,
            message: "Failed to commit transaction".to_string(),
        });
    }

    HttpResponse::Ok().json(BasicResponse {
        success: true,
        message: "Successfully added the campaign".to_string(),
    })
}

#[get("/list")]
pub async fn list_campaign(db: web::Data<MySqlPool>) -> impl Responder {
    let query = "SELECT * FROM campaign";

    let result = sqlx::query_as::<_, Campaign>(query)
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(operators) => HttpResponse::Ok().json(DataResponse {
            success: true,
            message: "Successfully fetched campaigns".to_string(),
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
pub async fn delete_campaign(db: web::Data<MySqlPool>, uuid: web::Path<String>) -> impl Responder {
    let mut transaction = db.begin().await.unwrap();

    let query = "DELETE FROM campaign WHERE uuid = ?";
    let result = sqlx::query(query)
        .bind(uuid.as_ref())
        .execute(&mut *transaction)
        .await;

    match result {
        Ok(_) => {
            let query2 = "DELETE FROM campaign_operator WHERE campaign_id = ?";
            let result2 = sqlx::query(query2)
                .bind(uuid.as_ref())
                .execute(&mut *transaction)
                .await;

            match result2 {
                Ok(_) => {
                    transaction.commit().await.unwrap();
                    
                    HttpResponse::Ok().json(BasicResponse {
                        success: true,
                        message: "Successfully delete the data..".to_string(),
                    })
                }
                Err(e) => {
                    error!("{}", e);

                    transaction.rollback().await.unwrap();

                    HttpResponse::InternalServerError().json(BasicResponse {
                        success: false,
                        message: "Something went wrong".to_string(),
                    })
                }
            }
        }
        Err(e) => {
            // If the first delete fails, rollback the transaction
            transaction.rollback().await.unwrap();
            HttpResponse::InternalServerError().json(format!("Failed to delete campaign: {}", e))
        }
    }
}