use crate::AppState;
use axum::extract::{State,Request};
use axum::http::{StatusCode,header};
use axum::response::Response;
use axum::middleware::Next;

pub async fn auth_middleware(State(state): State<AppState>, mut request: Request,    
next: Next,
) -> Result<Response,StatusCode>{
    let auth = request.headers()
    .get(header::AUTHORIZATION)
    .and_then(|value | value.to_str().ok())
    .filter(|value|value.starts_with("Bearer"))
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = state.jwt_service.verify_token(auth).map_err(|_| StatusCode::UNAUTHORIZED)?;
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)

}