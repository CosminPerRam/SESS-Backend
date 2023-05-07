use std::sync::Arc;
use warp::Filter;
use crate::schema::schema;
use crate::start_warp::graphql::{endpoint, serve_schema, websocket_protocol_header};
use crate::start_warp::homepage::homepage;
use crate::start_warp::playground::playground;

pub async fn start_warp() {
    let root_node = Arc::new(schema());

    let log = warp::log("warp_subscriptions");

    let routes = warp::path("subscriptions")
        .and(warp::ws())
        .map(serve_schema(root_node))
        .map(websocket_protocol_header)
        .or(endpoint(schema()))
        .or(playground())
        .or(homepage())
        .with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
