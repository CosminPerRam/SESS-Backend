mod setup_log;
mod start_warp;

use crate::setup_log::setup_log;
use crate::start_warp::start_warp;

#[tokio::main]
async fn main() {
    setup_log();

    log::info!("Listening on 127.0.0.1:8080");

    start_warp().await;
}
