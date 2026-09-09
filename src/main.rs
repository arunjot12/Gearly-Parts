use tokio::net::TcpListener;
use axum::{Json, Router, routing::{get,post}, serve};
pub mod auth;
pub mod parts;
pub mod db;
pub mod model;
pub mod schema;
use crate::{auth::JwtService, db::{create_pool,DbPool}, parts::api::create_part};

#[derive(Clone)]
struct AppState {
    db_pool: DbPool,
    jwt_service: JwtService
}

#[tokio::main]
async fn main(){

    let jwt = std::env::var("JwtService").expect("JWT secret needs to set");

    let jwt_service = JwtService::new(&jwt);
    let pool = create_pool();

    let state = AppState { db_pool: pool, jwt_service: jwt_service };
    let app = Router::new().
    route("/create_product",post(create_part)).with_state(state);

    let port: u16 = std::env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(3000);

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
    .await
    .unwrap_or_else(|e| panic!("failed to bind to port {port}: {e}"));

    serve(listener, app).await.unwrap();

}     