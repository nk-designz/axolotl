use anyhow::Error;
use axum::{
    routing::{get, head, patch, post, put},
    Router,
};

use super::super::handler::{exists, finish, index, root, upload, uploads};

#[derive(Clone, PartialEq, Builder)]
pub struct Server {
    pub host: String,
    pub port: usize,
}

impl Server {
    pub async fn run(&self) -> Option<Error> {
        let app = Router::new()
            .route("/", get(index::handler))
            .route("/v2", get(root::handler))
            .route("/v2/:name/blobs/:digest", head(exists::handler))
            .route("/v2/:name/blobs/uploads", post(uploads::handler))
            .route("/v2/:name/blobs/uploads/:uuid", patch(upload::handler))
            .route("/v2/:name/blobs/uploads/:uuid", put(finish::handler))
            .route("/v2/:name/manifests/:reference", head(index::handler))
            .route("/v2/:name/manifests/:reference", put(index::handler))
            .route("/v2/:name/blobs/:digest", get(index::handler))
            .route("/v2/:name/manifests/:reference", get(index::handler));

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
