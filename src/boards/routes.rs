use crate::{boards::{Board, Boards}, error_handler::AppError};
use serde_json::{json, Value};
use axum::{
    Router,
    routing::get,
    extract::Path,
    response::Json
};

async fn all() -> Result<Json<Vec<Boards>>, AppError> {
    let boards = Boards::all()?;
    Ok(Json(boards))
}

async fn find(Path(id): Path<i32>) -> Result<Json<Boards>, AppError> {
    let board = Boards::find(id)?;
    Ok(Json(board))
}

async fn create(Json(board): Json<Board>) -> Result<Json<Boards>, AppError> {
    let board = Boards::create(board)?;
    Ok(Json(board))
}

async fn update(
    Path(id): Path<i32>,
    Json(board): Json<Board>
    // Path(( id, board )): Path<(i32, Board)>
) -> Result<Json<Boards>, AppError> {
    let board = Boards::update(id, board)?;
    Ok(Json(board))
}

async fn destroy(Path(id): Path<i32>) -> Result<Json<Value>, AppError> {
    let deleted_board = Boards::delete(id)?;
    Ok(Json(json!({ "deleted": deleted_board })))
}

pub fn init_routes() -> Router {
     Router::new()
        .route("/boards", get(all).post(create))
        .route("/boards/:id", get(find).put(update).delete(destroy))
}
