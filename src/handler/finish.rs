use axum::{
    body::Bytes,
    extract,
    http::header::{HeaderMap, HeaderValue},
    response::{Headers, IntoResponse},
};
use http::StatusCode;
use serde::Deserialize;
use std::fs::{rename, OpenOptions};
use std::io::{prelude::*, SeekFrom};
use std::path::Path;

const LAYER_PATH: &'static str = "/tmp/layers";

#[derive(Deserialize)]
pub struct RequestQuery {
    digest: String,
}

pub async fn handler(
    extract::Path((name, uuid)): extract::Path<(String, String)>,
    rq: extract::Query<RequestQuery>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
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
