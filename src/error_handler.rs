use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;

#[derive(Debug)]
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                json!({
                    "status": "error",
                    "message": format!("Something went wrong: {}", self.0)
                })
            )
        ).into_response()
    }
}


// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}


// We create our own rejection type
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

// We implement `From<JsonRejection> for ApiError`
// impl From<JsonRejection> for ApiError {
//     fn from(rejection: JsonRejection) -> Self {
//         Self {
//             status: rejection.status(),
//             message: rejection.body_text(),
//         }
//     }
// }

// We implement `IntoResponse` so `ApiError` can be used as a response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "status": "error",
            "message": self.message,
            "origin": "derive_from_request"
        });

        (self.status, axum::Json(payload)).into_response()
    }
}
