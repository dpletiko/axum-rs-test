use crate::{widgets::{Widget, Widgets}, error_handler::AppError};
use serde_json::{json, Value};
use axum::{
    Router,
    routing::get,
    extract::Path,
    response::Json
};

async fn all() -> Result<Json<Vec<Widgets>>, AppError> {
    let widgets = Widgets::all()?;
    Ok(Json(widgets))
}

async fn find(Path(id): Path<i32>) -> Result<Json<Widgets>, AppError> {
    let widget = Widgets::find(id)?;
    Ok(Json(widget))
}

async fn create(Json(widget): Json<Widget>) -> Result<Json<Widgets>, AppError> {
    let widget = Widgets::create(widget)?;
    Ok(Json(widget))
}

async fn update(
   Path(id): Path<i32>,
   Json(widget): Json<Widget>
) -> Result<Json<Widgets>, AppError> {
    let widget = Widgets::update(id, widget)?;
    Ok(Json(widget))
}

async fn destroy(Path(id): Path<i32>) -> Result<Json<Value>, AppError> {
    let deleted_widget = Widgets::delete(id)?;
    Ok(Json(json!({ "deleted": deleted_widget })))
}

pub fn init_routes() -> Router {
     Router::new()
        .route("/widgets", get(all).post(create))
        .route("/widgets/:id", get(find).put(update).delete(destroy))
}
