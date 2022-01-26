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

const LAYER_PATH: &'static str = "/tmp/layers";

pub async fn handler(
    extract::Path((name, uuid)): extract::Path<(String, String)>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
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
