use std::collections::HashMap;
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
enum ApiStatus {
    FAIL,
    ERROR,
    SUCCESS
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    status: ApiStatus,
    data: Option<T>,
    errors: Option<HashMap<String, Vec<String>>>,
    message: Option<String>
}


impl<T> ApiResponse<T> {
    pub fn success(data: T, message: Option<String>) -> Self {
        Self { status: ApiStatus::SUCCESS, data: Some(data), message, errors: None }
    }

    pub fn error(message: String) -> Self {
        Self { status: ApiStatus::ERROR, message: Some(message), data: None, errors: None }
    }

    pub fn fail(errors: HashMap<String, Vec<String>>, message: Option<String>) -> Self {
        Self { status: ApiStatus::FAIL, errors: Some(errors), message, data: None }
    }
}
