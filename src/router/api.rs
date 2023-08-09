use axum::body::{Body, BoxBody};
use axum::http::{Request, Response};
use axum::response::IntoResponse;
use axum::{Router, http::StatusCode, Json};
use serde_json::json;
use crate::{auth, users, boards, widgets};

pub fn init_router() -> Router {
    Router::new()
        .merge(auth::init_routes())
        .merge(users::init_routes())
        .merge(boards::init_routes())
        .merge(widgets::init_routes())
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
