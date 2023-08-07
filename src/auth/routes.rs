use crate::{
    error_handler::AppError,
    users::Users,
    auth::Authenticable,
    utils::ApiResponse
};
use axum::{
    response::Json,
    routing::{get, post},
    Router
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCodeRequest {
    email: String,
}
impl AuthCodeRequest {
    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    email: String,
    pin: String
}
impl AuthRequest {
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn pin(&self) -> &str{
        &self.pin
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: Users,
    pub token: String,
    pub permissions: Option<Vec<String>>
}


async fn email(Json(auth): Json<AuthCodeRequest>) -> Result<Json<ApiResponse<AuthCodeRequest>>, AppError> {
    let user = Users::request_auth_code(auth.into())?;

    Ok(Json(ApiResponse::success(user, Some("Check your email for the verification pin code".to_string()))))
}

async fn login(Json(auth): Json<AuthRequest>) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let user = Users::login(auth.into())?;

    Ok(Json(ApiResponse::success(user, Some("Successfully logged in!".to_string()))))
}

async fn logout() -> Result<Json<ApiResponse<Option<()>>>, AppError> {
    let user = Users::logout()?;

    Ok(Json(ApiResponse::success(None, Some("Successfully logged out!".to_string()))))
}

async fn profile() -> Result<Json<Option<()>>, AppError> {
    let user = Users::profile()?;
    Ok(Json(user))
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/auth/email", post(email))
        .route("/auth/login", post(login))
        .route("/auth/logout", get(logout).post(logout))
}
