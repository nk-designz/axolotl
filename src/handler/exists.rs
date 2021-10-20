use axum::{
    extract,
    response::{Headers, IntoResponse},
};
use http::StatusCode;
use std::path::Path;

const LAYER_PATH: &'static str = "/tmp/layers";

pub async fn handler(
    extract::Path(name): extract::Path<String>,
    extract::Path(digest): extract::Path<String>,
) -> impl IntoResponse {
    let docker_content_digest = digest.clone();

    println!("Does {0} exists for {1}?", name, docker_content_digest);

    let hash: String = match digest.split(":").last() {
        Some(hash) => hash.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Headers(vec![("docker-content-digest", docker_content_digest)]),
            );
        }
    };
    let path_string = format!("{0}/{1}", LAYER_PATH, hash);
    let path = Path::new(&path_string);
    // Check if digest exists
    if !path.exists() {
        return (
            StatusCode::NOT_FOUND,
            Headers(vec![("docker_content_digest", docker_content_digest)]),
        );
    }
    let cl = std::fs::metadata(path).unwrap().len().to_string();
    (
        StatusCode::OK,
        Headers(vec![
            ("content-length", cl),
            ("docker-content-digest", docker_content_digest),
        ]),
    )
}
