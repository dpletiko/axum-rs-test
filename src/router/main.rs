use axum::Router;
use axum::body::Body;
use axum::http::Request;
use tower::service_fn;
use std::convert::Infallible;
use super::{api, web};

pub fn init() -> Router {
    Router::new()
        .nest("/", web::init_router())
        .nest("/api", api::init_router())
        // .fallback(fallback)
        .fallback_service(
            service_fn(|req: Request<Body>| async move {
                let response = match req.uri().path().starts_with("/api") {
                    true  => api::fallback(req),
                    false => web::fallback(req)
                };

                Ok::<_, Infallible>(response)
            })
        )
}


// async fn fallback(uri: Uri) -> (StatusCode, String | Html<&'static str>) {
//     match uri.path().split("/").next().unwrap_or("") {
//         "api" => (StatusCode::NOT_FOUND, format!("Route Not Found! [{}]", uri)),
//
//         _ => (StatusCode::NOT_FOUND, Html("<p>Hello, World!</p>"))
//     }
// }
