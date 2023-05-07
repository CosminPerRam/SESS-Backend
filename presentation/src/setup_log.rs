use std::env;

pub fn setup_log() {
    env::set_var("RUST_LOG", "warp_subscriptions");
    env_logger::init();
}
