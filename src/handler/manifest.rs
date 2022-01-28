use super::super::schema::manifest;
use axum::{
    extract,
    response::{Headers, IntoResponse},
};
use http::StatusCode;
use serde_json;
use sha256::digest_file;
use std::path::Path;

const MANIFEST_PATH: &'static str = "/tmp/manifests";

pub async fn exists(
    extract::Path((name, reference)): extract::Path<(String, String)>,
) -> impl IntoResponse {
    println!("Does {0} exists for {1}?", name, reference);

    let hash: String = match reference.split(":").last() {
        Some(hash) => hash.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Headers(vec![("docker-content-digest", reference)]),
            );
        }
    };
    let path_string = format!("{0}/{1}", MANIFEST_PATH, hash);
    let path = Path::new(&path_string);
    // Check if digest exists
    if !path.exists() {
        return (
            StatusCode::NOT_FOUND,
            Headers(vec![("docker-content-digest", reference)]),
        );
    }
    let content_length = std::fs::metadata(path).unwrap().len().to_string();
    let manifest_data = std::fs::read_to_string(path).unwrap();
    let manifest: manifest::Manifest = serde_json::from_str(&manifest_data).unwrap();
    println!("{:?}", manifest);
    let media_type = manifest.media_type;
    let digest = digest_file(path).unwrap();
    let docker_content_digest = format!("sha256:{0}", digest);
    (
        StatusCode::OK,
        Headers(vec![
            ("content-length", content_length),
            ("docker-content-digest", docker_content_digest),
            ("content-type", media_type.to_string()),
        ]),
    )
}
