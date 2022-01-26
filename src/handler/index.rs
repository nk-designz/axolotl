use axum::response::IntoResponse;
use http::StatusCode;

pub async fn handler() -> impl IntoResponse {
    (StatusCode::OK, ">(._.)< Hello from axolotl!\n  (  )_")
}
