use axum::{
    extract::Path,
    response::{Headers, IntoResponse},
};
use getrandom;
use http::StatusCode;
use uuid;

pub async fn handler(Path(name): Path<String>) -> impl IntoResponse {
    let mut buffer = [0u8; 16];
    getrandom::getrandom(&mut buffer).unwrap();
    let guid = uuid::Builder::from_bytes(buffer)
        .build()
        .to_hyphenated()
        .to_string();
    (
        StatusCode::ACCEPTED,
        Headers(vec![
            ("location", format!("/v2/{0}/blobs/uploads/{1}", name, guid)),
            ("range", "0-0".to_string()),
            ("content-length", "0".to_string()),
            ("docker-upload-uuid", guid),
        ]),
    )
}
