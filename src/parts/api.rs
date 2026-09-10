use crate::{AppState, model::NewProduct, parts::handler::handle_product_insertion};
use axum::{Json, extract::State, http::StatusCode};

#[axum::debug_handler]
pub async fn create_part(
    State(state): State<AppState>,
    Json(payload) : Json<NewProduct>
)
-> Result<(StatusCode, String), (StatusCode, String)> {

     let connection = state
        .db_pool
        .get()
        .await
        .expect("Failed to get DB connection from pool");

    let result = connection.
        interact(
        move | connection | handle_product_insertion(connection, payload))
        .await 
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,e.to_string()))?;

      match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            "successfully created shopkeeper".to_string(),
        )),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}
