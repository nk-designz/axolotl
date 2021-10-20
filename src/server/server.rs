use anyhow::Error;
use axum::{
    handler::{get, head},
    Router,
};

use super::super::handler::exists;

#[derive(Clone, PartialEq, Builder)]
pub struct Server {
    pub host: String,
    pub port: usize,
}

impl Server {
    pub async fn run(&self) -> Option<Error> {
        let app = Router::new()
            .route("/", get(index))
            .route("/v2/:name/blobs/:digest", head(exists::handler));

        match axum::Server::bind(match &format!("{0}:{1}", self.host, self.port).parse() {
            Ok(sock) => sock,
            Err(err) => return Some(Error::new(err.clone())),
        })
        .serve(app.into_make_service())
        .await
        {
            Ok(_) => None,
            Err(err) => Some(Error::new(err)),
        }
    }
}

async fn index() -> String {
    "Hello World".to_string()
}
