use std::env;
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

    let server_port: u16 = env::var("PORT").unwrap_or("8080".to_string()).parse().unwrap_or(8080);

    log::info!("Server started on port ${server_port}");
    warp::serve(routes).run(([0, 0, 0, 0], server_port)).await;
}
