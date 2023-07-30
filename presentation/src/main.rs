mod setup_log;
mod start_warp;
mod schema;

use db::db_do;
use crate::setup_log::setup_log;
use crate::start_warp::start_warp;


#[tokio::main]
async fn main() {
    setup_log();

    db_do();

    //start_warp().await;
}
