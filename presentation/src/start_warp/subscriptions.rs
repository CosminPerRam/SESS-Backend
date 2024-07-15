use std::sync::Arc;
use std::time::Duration;
use juniper_graphql_ws::ConnectionConfig;
use warp::{Filter, Rejection, Reply};
use context::get_context;
use crate::schema::{schema};

pub fn subscriptions() -> impl Filter + Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let log = warp::log("warp_subscriptions");

    let root_node = Arc::new(schema());

    warp::path("subscriptions")
        .and(juniper_warp::subscriptions::make_ws_filter(
            root_node,
            ConnectionConfig::new(get_context())
                .with_keep_alive_interval(Duration::from_secs(2)),
        ))
        .with(log)
}
