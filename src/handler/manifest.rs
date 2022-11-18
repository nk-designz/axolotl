use super::super::schema::manifest;
use super::super::schema::media_type::MediaType;
use axum::{
    body::Bytes,
    extract,
    response::{Headers, IntoResponse},
};
use http::StatusCode;
use serde_json;
use sha256::digest_file;
use std::fs::{copy, read_to_string, OpenOptions};
use std::io::{prelude::*, SeekFrom};
use std::path::Path;

const MANIFEST_PATH: &'static str = "/tmp/manifests";

pub async fn exists(
    extract::Path((name, reference)): extract::Path<(String, String)>,
) -> impl IntoResponse {
    println!("Does {0} exists for {1}?", name, reference);
    log::info!("Request: File: {0} (line: {1})", file!(), line!());
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

pub async fn save(
    extract::Path((name, reference)): extract::Path<(String, String)>,
    body: Bytes,
) -> impl IntoResponse {
    log::info!("Request: File: {0} (line: {1})", file!(), line!());
    let manifest_path_string = format!("{0}/{1}.{2}.json", MANIFEST_PATH, name, reference);
    let manifest_path = Path::new(&manifest_path_string);

    let mut manifest_file = match OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&manifest_path)
    {
        Err(why) => panic!("couldn't open {}: {}", manifest_path.display(), why),
        Ok(file) => file,
    };

    manifest_file.seek(SeekFrom::Start(0)).unwrap();
    manifest_file.write_all(&body).unwrap();

    let hash = digest_file(&manifest_path_string).unwrap();

    copy(
        manifest_path_string,
        format!("{0}/{1}.json", MANIFEST_PATH, hash),
    )
    .unwrap();

    (
        StatusCode::CREATED,
        Headers(vec![("docker_content_digest", format!("sha256:{0}", hash))]),
        format!("/v2/{0}/manifests/{1}", name, reference),
    )
}

pub async fn get(
    extract::Path((name, reference)): extract::Path<(String, String)>,
    headers: http::header::HeaderMap,
) -> impl IntoResponse {
    log::info!("Request: File: {0} (line: {1})", file!(), line!());
    log::info!("{:?}", headers);
    let hash = reference.split(":").last().unwrap();
    let manifest_path_string = format!("{0}/{1}.{2}.json", MANIFEST_PATH, name, reference);
    let manifest_path = Path::new(&manifest_path_string);

    let hash_path_string = format!("{0}/{1}.json", MANIFEST_PATH, hash);
    let hash_path = Path::new(&hash_path_string);
    let path: String;

    if manifest_path.exists() {
        path = manifest_path_string;
    } else if hash_path.exists() {
        path = hash_path_string;
    } else {
        return (
            StatusCode::NOT_FOUND,
            Headers(vec![("reference", reference)]),
            "".to_string(),
        );
    }

    let manifest_string: String = match read_to_string(&path) {
        Ok(manifest_content) => manifest_content,
        Err(err) => panic!("Reading file {0} failed: {1}", &path, err),
    }
    .parse()
    .unwrap();

    let content: manifest::ImageManifest = match serde_json::from_str(&manifest_string) {
        Ok(content) => content,
        Err(err) => panic!("{}", err),
    };

    let _content_length = Path::new(&path).metadata().unwrap().len();

    let digest = digest_file(path).unwrap();
    (
        StatusCode::OK,
        Headers(vec![
            ("docker-content-digest", format!("sha256:{0}", digest)),
            (
                "media-type",
                content
                    .media_type
                    .clone()
                    .unwrap_or(MediaType::Default)
                    .to_string(),
            ),
        ]),
        serde_json::to_string(&content).unwrap(),
    )
}
