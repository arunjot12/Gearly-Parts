use tokio::net::TcpListener;
use axum::{Json, Router, routing::{get,post}, serve};
pub mod auth;
pub mod parts;
pub mod db;
use crate::parts::create_product::create_part;

#[tokio::main]
async fn main(){
    let app = Router::new().
    route("/create_product",post(create_part));

    let port: u16 = std::env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(3000);

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
    .await
    .unwrap_or_else(|e| panic!("failed to bind to port {port}: {e}"));

    serve(listener, app).await.unwrap();

}     