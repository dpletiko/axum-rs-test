use crate::{
    models::board::{CreateBoard, Board},
    error_handler::AppError
};
use serde_json::{json, Value};
use axum::{
    Router,
    routing::get,
    extract::Path,
    response::Json
};

async fn all() -> Result<Json<Vec<Board>>, AppError> {
    let boards = Board::all()?;
    Ok(Json(boards))
}

async fn find(Path(id): Path<i32>) -> Result<Json<Board>, AppError> {
    let board = Board::find(id)?;
    Ok(Json(board))
}

async fn create(Json(board): Json<CreateBoard>) -> Result<Json<Board>, AppError> {
    let board = Board::create(board)?;
    Ok(Json(board))
}

async fn update(
    Path(id): Path<i32>,
    Json(board): Json<CreateBoard>
    // Path(( id, board )): Path<(i32, Board)>
) -> Result<Json<Board>, AppError> {
    let board = Board::update(id, board)?;
    Ok(Json(board))
}

async fn destroy(Path(id): Path<i32>) -> Result<Json<Value>, AppError> {
    let deleted_board = Board::delete(id)?;
    Ok(Json(json!({ "deleted": deleted_board })))
}

pub fn init_routes() -> Router {
     Router::new()
        .route("/boards", get(all).post(create))
        .route("/boards/:id", get(find).put(update).delete(destroy))
}
