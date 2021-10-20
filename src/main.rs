#[macro_use]
extern crate derive_builder;
use tokio::{signal, task};

mod server;
use server::server::ServerBuilder;
pub mod handler;

#[tokio::main]
async fn main() {
    // Start registry server
    task::spawn(registry());
    // Stop on ctr-c
    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }
}

async fn registry() {
    let server = ServerBuilder::default()
        .host("0.0.0.0".to_string())
        .port(3000)
        .build()
        .unwrap();
    server.run().await;
}
