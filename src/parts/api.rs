use crate::{model::NewProduct, AppState};
use axum::{Json, extract::State, http::StatusCode};


#[axum::debug_handler]
pub async fn create_part(
    State(state): State<AppState>,
    Json(payload) : Json<NewProduct>
){

}