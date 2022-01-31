use super::super::handler::{index, layer, manifest, root};
use anyhow::Error;
use axum::{
    routing::{get, head, patch, post, put},
    Router,
};

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
            .route("/v2/:name/blobs/:digest", head(layer::exists))
            .route("/v2/:name/blobs/uploads", post(layer::start_upload))
            .route("/v2/:name/blobs/uploads/:uuid", patch(layer::upload))
            .route("/v2/:name/blobs/uploads/:uuid", put(layer::finish_upload))
            .route("/v2/:name/manifests/:reference", head(manifest::exists))
            .route("/v2/:name/manifests/:reference", put(manifest::save))
            .route("/v2/:name/blobs/:digest", get(layer::get))
            .route("/v2/:name/manifests/:reference", get(manifest::get));

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
