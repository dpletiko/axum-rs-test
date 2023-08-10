use crate::{
    error_handler::AppError,
    models::user::{CreateUser, User},
};
use axum::{extract::Path, Json, routing::get, Router, Extension};
use serde_json::{json, Value};

async fn all() -> Result<Json<Vec<User>>, AppError> {
    let users = User::all()?;
    Ok(Json(users))
}

async fn find(Path(id): Path<i32>) -> Result<Json<User>, AppError> {
    let user = User::find(id)?;
    Ok(Json(user))
}

async fn create(Json(user): Json<CreateUser>) -> Result<Json<User>, AppError> {
    let user = User::create(user)?;
    Ok(Json(user))
}

async fn update(
    Path(id): Path<i32>,
    Json(user): Json<CreateUser>
) -> Result<Json<User>, AppError> {
    let user = User::update(id, user)?;
    Ok(Json(user))
}

async fn destroy(Path(id): Path<i32>) -> Result<Json<Value>, AppError> {
    let deleted_user = User::delete(id)?;
    Ok(Json(json!({ "deleted": deleted_user })))
}


async fn profile(
    Extension(user): Extension<User>,
) -> Result<Json<User>, AppError> {
    // let user = User::profile()?;
    Ok(Json(user))
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/profile", get(profile))

        .route("/users", get(all).post(create))
        .route("/users/:id", get(find).put(update).delete(destroy))
}
