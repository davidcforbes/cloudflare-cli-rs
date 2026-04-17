use log::error;
use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = cfad::runner::run().await {
        error!("Error: {}", e);
        eprintln!("{}", e);
        process::exit(1);
    }
}
