use axum::response::IntoResponse;
use http::StatusCode;

pub async fn handler() -> impl IntoResponse {
    log::info!("Request: File: {0} (line: {1})", file!(), line!());
    StatusCode::OK
}
