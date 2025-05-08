use crate::{dto::*, model::*};

use actix_web::{delete, get, http::header, patch, post, web, HttpResponse, Responder};

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
            (uuid, campaign_id, operator_id, grade, handle, created_at, updated_at)
            VALUES (?, ?, ?, ?, 0, NOW(), NOW())
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

#[patch("/update/{uuid}")]
pub async fn update_campaign(
    db: web::Data<MySqlPool>,
    uuid: web::Path<String>,
    payload: web::Json<AddCampaignRequest>,
) -> impl Responder {
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

    // Update campaign data
    let result = sqlx::query(
        "
        UPDATE campaign
        SET name = ?, message = ?, slug = ?, updated_at = NOW()
        WHERE uuid = ?
        ",
    )
    .bind(&payload.name)
    .bind(&payload.message)
    .bind(&payload.slug)
    .bind(uuid.as_ref())
    .execute(&mut *tx)
    .await;

    if let Err(e) = result {
        error!("Failed to update campaign: {}", e);
        tx.rollback().await.ok();
        return HttpResponse::InternalServerError().json(BasicResponse {
            success: false,
            message: "Failed to update campaign".to_string(),
        });
    }

    // Remove all record campaign_operator by campaign_id
    let resultdel = sqlx::query("DELETE FROM campaign_operator WHERE campaign_id = ?")
        .bind(uuid.as_ref())
        .execute(&mut *tx)
        .await;

    if let Err(e) = resultdel {
        error!("Failed to delete campaign_operator: {}", e);
        tx.rollback().await.ok();
        return HttpResponse::InternalServerError().json(BasicResponse {
            success: false,
            message: "Failed to delete campaign_operator".to_string(),
        });
    }

    // Insert back data operators
    for op in &payload.operators {
        let result = sqlx::query(
            "
            INSERT INTO campaign_operator
            (uuid, campaign_id, operator_id, grade, handle, created_at, updated_at)
            VALUES (?, ?, ?, ?, 0, NOW(), NOW())
        ",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(uuid.as_ref())
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
        message: "Successfully update the campaign".to_string(),
    })
}

#[get("/get/{uuid}")]
pub async fn get_campaign_by_id(
    db: web::Data<MySqlPool>,
    uuid: web::Path<String>,
) -> impl Responder {
    let query = "SELECT * FROM campaign WHERE uuid = ?";

    let result = sqlx::query_as::<_, Campaign>(query)
        .bind(uuid.as_ref())
        .fetch_optional(db.get_ref())
        .await;

    let query2 = "SELECT operator_id as id, grade FROM campaign_operator WHERE campaign_id = ?";

    let result2 = sqlx::query_as::<_, CampaignOperator>(query2)
        .bind(uuid.as_ref())
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(Some(campaign)) => {
            let operators = result2.unwrap_or_default();

            let response_data = serde_json::json!({
                "uuid": campaign.uuid,
                "name": campaign.name,
                "message": campaign.message,
                "slug": campaign.slug,
                "operators": operators
            });

            HttpResponse::Ok().json(DataResponse {
                success: true,
                message: "Successfully fetched campaign with operators".to_string(),
                data: Some(response_data),
            })
        }
        Ok(None) => {
            // Campaign not found
            HttpResponse::NotFound().json(BasicResponse {
                success: false,
                message: "Campaign not found".to_string(),
            })
        }
        Err(e) => {
            // Handle error in fetching campaign
            error!("{}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "message": "Something went wrong",
                "error_details": e.to_string()
            }))
        }
    }
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

#[get("/{slug}")]
pub async fn get_campaign_by_slug(
    db: web::Data<MySqlPool>,
    slug: web::Path<String>,
) -> impl Responder {
    // Here get the operator , grade, and handle times
    let query = r#"
        SELECT o.identity, co.grade, co.handle, co.uuid
        FROM operator o
        JOIN campaign_operator co ON co.operator_id = o.uuid
        JOIN campaign c ON c.uuid = co.campaign_id
        WHERE c.slug = ? AND o.status = 1
    "#;

    let result = sqlx::query_as::<_, CampaignSlug>(query)
        .bind(slug.into_inner())
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(operators) => {
            // Round roubin weight algorithm
            // Get data that handle != grade and get one data that have max grade
            let best = operators
                .iter()
                .filter(|op| op.handle != op.grade)
                .max_by_key(|op| op.grade)
                .unwrap();

            // Increase handle
            let update_query = r#"
                UPDATE campaign_operator
                SET handle = handle + 1
                WHERE uuid = ?
            "#;

            let _ = sqlx::query_as::<_, CampaignSlug>(update_query)
                .bind(&best.uuid)
                .fetch_all(db.get_ref())
                .await;

            // Get total grade & total handle for all row operators (not best)
            let (total_grade, total_handle): (i32, i32) = operators
                .iter()
                .fold((0, 0), |(sum_grade, sum_handle), op| {
                    (sum_grade + op.grade, sum_handle + op.handle)
                });

            // If total grade ==  total handle + 1 or means we need to reset all handle
            if total_grade == total_handle + 1 {
                for op in &operators {
                    let update_query = r#"
                        UPDATE campaign_operator
                        SET handle = 0
                        WHERE uuid = ?
                    "#;

                    let _ = sqlx::query(update_query)
                        .bind(&op.uuid)
                        .execute(db.get_ref())
                        .await;
                }
            }

            HttpResponse::Found()
                .append_header((header::LOCATION, best.identity.clone()))
                .finish()
        }
        Err(e) => {
            error!("DB error: {}", e);
            HttpResponse::InternalServerError().json(BasicResponse {
                success: false,
                message: "Failed to fetch operators".to_string(),
            })
        }
    }
}
