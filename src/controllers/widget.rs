use crate::{
    models::widget::{CreateWidget, Widget},
    error_handler::AppError
};
use serde_json::{json, Value};
use axum::{
    Router,
    routing::get,
    extract::Path,
    response::Json
};

async fn all() -> Result<Json<Vec<Widget>>, AppError> {
    let widgets = Widget::all()?;
    Ok(Json(widgets))
}

async fn find(Path(id): Path<i32>) -> Result<Json<Widget>, AppError> {
    let widget = Widget::find(id)?;
    Ok(Json(widget))
}

async fn create(Json(widget): Json<CreateWidget>) -> Result<Json<Widget>, AppError> {
    let widget = Widget::create(widget)?;
    Ok(Json(widget))
}

async fn update(
   Path(id): Path<i32>,
   Json(widget): Json<CreateWidget>
) -> Result<Json<Widget>, AppError> {
    let widget = Widget::update(id, widget)?;
    Ok(Json(widget))
}

async fn destroy(Path(id): Path<i32>) -> Result<Json<Value>, AppError> {
    let deleted_widget = Widget::delete(id)?;
    Ok(Json(json!({ "deleted": deleted_widget })))
}

pub fn init_routes() -> Router {
     Router::new()
        .route("/widgets", get(all).post(create))
        .route("/widgets/:id", get(find).put(update).delete(destroy))
}
