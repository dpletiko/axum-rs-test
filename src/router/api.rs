use axum::body::{Body, BoxBody};
use axum::http::{Request, Response};
use axum::response::IntoResponse;
use axum::{Router, http::StatusCode, Json};
use serde_json::json;
use crate::controllers::{auth, user, board, widget};
use crate::models::User;
use crate::utils::{AuthError, Claims};
use axum::middleware::{from_fn, Next};

pub fn init_router() -> Router {
    Router::new()
        .merge(user::init_routes())
        .merge(widget::init_routes())
        .merge(board::init_routes())
        .layer(from_fn(auth_middleware))
        .merge(auth::init_routes())

        // .fallback(api_fallback)
}

// async fn api_fallback(uri: Uri) -> (StatusCode, String) {
//     (StatusCode::NOT_FOUND, format!("Route Not Found! [{}]", uri))
// }

pub fn fallback(req: Request<Body>) -> Response<BoxBody> {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "status": "error",
            "message": format!("Route Not Found! `{} {}`", req.method(), req.uri().path())
        }))
    ).into_response()
}

async fn auth_middleware<B>(
    claims: Claims,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, AuthError> {
    let user = User::find(claims.sub)
        .map_err(|_| AuthError::InvalidToken)?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
