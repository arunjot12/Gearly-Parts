use crate::{AppState, model::{Product,NewProduct}, product::handler::{handle_products,handle_product_insertion}};
use axum::{Json, extract::State, http::StatusCode};

#[axum::debug_handler]
pub async fn create_part(
    State(state): State<AppState>,
    Json(payload): Json<NewProduct>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let connection = state
        .db_pool
        .get()
        .await
        .expect("Failed to get DB connection from pool");

    if payload.price <= 0 {
        return Err((StatusCode::BAD_REQUEST,"Add a valid price".to_string()))
    }

    let result = connection
        .interact(move |connection| handle_product_insertion(connection, payload))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            "successfully created product".to_string(),
        )),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

#[axum::debug_handler]
pub async fn get_products(State(state): State<AppState>) -> Result<Json<Vec<Product>>, String>{

    let connection = state.db_pool.get().await.expect("Failed to get DB connection from pool");

    let result = connection.interact(
       move |connection | handle_products(connection) )
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    
    let result = match result {
        Ok(result) => result,
        Err(e) => return Err("BAD_REQUEST".to_string())
    };
    
    result
}