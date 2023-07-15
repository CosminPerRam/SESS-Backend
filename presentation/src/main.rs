mod setup_log;
mod start_warp;
mod schema;

use crate::setup_log::setup_log;
use crate::start_warp::start_warp;

#[tokio::main]
async fn main() {
    setup_log();

    start_warp().await;
}
