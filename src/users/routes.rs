use crate::{
    error_handler::AppError,
    users::{User, Users},
};
use axum::{extract::Path, response::Json, routing::get, Router};
use serde_json::{json, Value};

async fn all() -> Result<Json<Vec<Users>>, AppError> {
    let users = Users::all()?;
    Ok(Json(users))
}

async fn find(Path(id): Path<i32>) -> Result<Json<Users>, AppError> {
    let user = Users::find(id)?;
    Ok(Json(user))
}

async fn create(Json(user): Json<User>) -> Result<Json<Users>, AppError> {
    let user = Users::create(user)?;
    Ok(Json(user))
}

async fn update(
    Path(id): Path<i32>,
    Json(user): Json<User>
) -> Result<Json<Users>, AppError> {
    let user = Users::update(id, user)?;
    Ok(Json(user))
}

async fn destroy(Path(id): Path<i32>) -> Result<Json<Value>, AppError> {
    let deleted_user = Users::delete(id)?;
    Ok(Json(json!({ "deleted": deleted_user })))
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/users", get(all).post(create))
        .route("/users/:id", get(find).put(update).delete(destroy))
}
