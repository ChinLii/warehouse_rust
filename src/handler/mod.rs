pub mod helper;
use axum::{
    extract:: {Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::model::Store;
use crate::model::dto::{StorePayload, ErrorResponse, OkResponse};

type DbPool = Pool<Postgres>;

pub async fn get_all_stores(State(pool): State<DbPool>) -> Result<Json<Vec<Store>>, StatusCode> {
    let stores = sqlx::query_as!(Store, "SELECT * FROM stores")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    return Ok(Json(stores));
}

pub async fn create_store(State(pool): State<DbPool>, Json(payload): Json<StorePayload>) -> Result<(StatusCode, Json<Store>), StatusCode>{
    let id: Uuid = Uuid::new_v4();
    let store = Store {
        id,
        name: payload.name.clone(),
        address: payload.address.clone(),
        phone: payload.phone.clone(),
    };
    sqlx::query!("INSERT INTO stores (id ,name, address, phone) VALUES ($1, $2, $3, $4)",id , store.name, store.address, store.phone)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    return Ok((StatusCode::CREATED, Json(store)));
}

pub async fn get_store_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<Store>, (StatusCode, Json<ErrorResponse>)> {

    if helper::validate_uuid(&id) == false{
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                status: 400,
                message: "Invalid UUID format".to_string(),
            }),
        ));
    }
    let uuid = Uuid::parse_str(&id).unwrap();

    // Fetch store from database
    let store = sqlx::query_as!(Store, "SELECT * FROM stores WHERE id = $1", uuid)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    status: 404,
                    message: "Store not found".to_string(),
                }),
            )
        })?;

    Ok(Json(store))
}

pub async fn update_store_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(payload): Json<StorePayload>,
) -> Result<Json<Store>, (StatusCode, Json<ErrorResponse>)> {
    // check Uuid validation
    if helper::validate_uuid(&id) == false{
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                status: 400,
                message: "Invalid UUID format".to_string(),
            }),
        ));
    }
    let uuid = Uuid::parse_str(&id).unwrap();
    // Update store in Database
    let result = sqlx::query!(
        "UPDATE stores 
         SET name = COALESCE($1, name), 
             address = COALESCE($2, address), 
             phone = COALESCE($3, phone) 
         WHERE id = $4 
         RETURNING id, name, address, phone",
        payload.name,
        payload.address,
        payload.phone,
        uuid
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                status: 404,
                message: "Store not found".to_string(),
            }),
        )
    })?;
    let updated_store = Store {
        id: result.id,
        name: result.name,
        address: result.address,
        phone: result.phone,
    };

    Ok(Json(updated_store))
}

pub async fn delete_store_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<OkResponse>, (StatusCode, Json<ErrorResponse>)> {
    // check Uuid validation
    if helper::validate_uuid(&id) == false{
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                status: 400,
                message: "Invalid UUID format".to_string(),
            }),
        ));
    }
    let uuid = Uuid::parse_str(&id).unwrap();
    // Delete store from db 
    let result = sqlx::query!(
        "DELETE FROM stores WHERE id = $1",
        uuid
    )
    .execute(&pool)
    .await;

    match result {
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    status: 500,
                    message: format!("Error deleting store: {}", e.to_string()),
                }),
                ));
        }
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        status: 404,
                        message: "Store not found".to_string(),
                    }),
                ));
            }
            return Ok(Json(OkResponse {
                status: 200,
                message: format!("Store deleted successfully {} rows affected", res.rows_affected()),
            }));
        }
    }
}