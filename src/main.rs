#[macro_use]
extern crate derive_builder;
use tokio::{signal, task};

mod server;
use server::server::ServerBuilder;
pub mod handler;
pub mod schema;

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
    let host: String = "0.0.0.0".to_string();
    let port: usize = 3000;
    println!(
        "Starting registry on {0}:{1}.\n\tPress Ctrl-c to shutdown.",
        host, port,
    );
    let server = ServerBuilder::default()
        .host(host)
        .port(port)
        .build()
        .unwrap();
    server.run().await;
}
