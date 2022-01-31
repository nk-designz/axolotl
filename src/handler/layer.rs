const LAYER_PATH: &'static str = "/tmp/layers";

pub async fn exists(
    extract::Path((name, digest)): extract::Path<(String, String)>,
) -> impl IntoResponse {
    log::info!("Request: File: {0} (line: {1})", file!(), line!());
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

use getrandom;
use uuid;

pub async fn start_upload(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    log::info!("Request: File: {0} (line: {1})", file!(), line!());
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
use axum::{
    body::Bytes,
    extract,
    http::header::{HeaderMap, HeaderValue},
    response::{Headers, IntoResponse},
};
use http::StatusCode;
use std::fs::OpenOptions;
use std::io::{prelude::*, SeekFrom};
use std::path::Path;

pub async fn upload(
    extract::Path((name, uuid)): extract::Path<(String, String)>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    log::info!("Request: File: {0} (line: {1})", file!(), line!());
    let layer_uuid = uuid.clone();
    let default_content_range = HeaderValue::from_str("0-0").unwrap();

    println!("Upload layer {0} for {1}.", layer_uuid, name);

    let mut content_range = headers
        .get("content-range")
        .unwrap_or(&default_content_range)
        .to_str()
        .unwrap_or("0-0")
        .split("-");

    let content_range_start = content_range
        .next()
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0);

    let _content_range_end = content_range
        .next()
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0);

    let layer_path_string = format!("{0}/{1}", LAYER_PATH, uuid);
    let layer_path = Path::new(layer_path_string.as_str());

    let mut layer_file = match OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&layer_path)
    {
        Err(why) => panic!("couldn't open {}: {}", layer_path.display(), why),
        Ok(file) => file,
    };

    layer_file
        .seek(SeekFrom::Start(content_range_start))
        .unwrap();

    layer_file.write_all(&body).unwrap();

    let current_postion = layer_file.seek(SeekFrom::Current(0)).unwrap();

    let range_response = format!("0-{0}", current_postion);
    let location = format!("/v2/{0}/blobs/uploads/{1}", name, uuid);

    (
        StatusCode::ACCEPTED,
        Headers(vec![
            ("content-length", "0".to_string()),
            ("docker-upload-uuid", uuid),
            ("range", range_response),
            ("location", location),
            (
                "docker-distribution-api-version",
                "registry/2.0".to_string(),
            ),
        ]),
    )
}

use serde::Deserialize;
use std::fs::rename;

#[derive(Deserialize)]
pub struct RequestQuery {
    digest: String,
}

pub async fn finish_upload(
    extract::Path((name, uuid)): extract::Path<(String, String)>,
    rq: extract::Query<RequestQuery>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    log::info!("Request: File: {0} (line: {1})", file!(), line!());
    let layer_uuid = uuid.clone();
    let default_content_range = HeaderValue::from_str("0-0").unwrap();

    println!("Upload layer {0} for {1}.", layer_uuid, name);

    if headers
        .get("content-length")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u64>()
        .unwrap()
        != 0
    {
        let mut content_range = headers
            .get("content-range")
            .unwrap_or(&default_content_range)
            .to_str()
            .unwrap_or("0-0")
            .split("-");

        let content_range_start = content_range
            .next()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);

        let _content_range_end = content_range
            .next()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);

        let layer_path_string = format!("{0}/{1}", LAYER_PATH, uuid);
        let layer_path = Path::new(layer_path_string.as_str());

        let mut layer_file = match OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&layer_path)
        {
            Err(why) => panic!("couldn't open {}: {}", layer_path.display(), why),
            Ok(file) => file,
        };

        layer_file
            .seek(SeekFrom::Start(content_range_start))
            .unwrap();

        layer_file.write_all(&body).unwrap();
    }

    let rdg = rq.digest.clone();
    let raw_digest = rdg.split(":").last().unwrap();

    rename(
        format!("{0}/{1}", LAYER_PATH, uuid),
        format!("{0}/{1}", LAYER_PATH, raw_digest),
    )
    .unwrap();

    (
        StatusCode::CREATED,
        Headers(vec![("docker-content-digest", rdg.clone())]),
        format!("/v2/{0}/blobs/{1}", name, rdg),
    )
}

pub async fn get(
    extract::Path((_name, digest)): extract::Path<(String, String)>,
) -> impl IntoResponse {
    log::info!("Request: File: {0} (line: {1})", file!(), line!());
    let hash = digest.split(":").last().unwrap();
    let layer_path_string = format!("{0}/{1}", LAYER_PATH, hash);

    let layer_path = Path::new(&layer_path_string);

    if !layer_path.exists() {
        let empty: Vec<u8> = Vec::new();
        return (
            StatusCode::NOT_FOUND,
            Headers(vec![("docker-content-digest", digest)]),
            empty,
        );
    }

    let content_length = layer_path.metadata().unwrap().len();

    let mut file = std::fs::File::open(&layer_path_string).unwrap();
    let mut content_buffer = Vec::new();
    file.read_to_end(&mut content_buffer).unwrap();

    (
        StatusCode::OK,
        Headers(vec![("content-length", content_length.to_string())]),
        content_buffer,
    )
}
