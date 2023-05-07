use std::sync::Arc;
use warp::Filter;
use crate::schema::{Context, schema};
use crate::start_warp::graphql::{serve_schema, websocket_protocol_header};
use crate::start_warp::homepage::homepage;
use crate::start_warp::playground::playground;

pub async fn start_warp() {
    let qm_schema = schema();
    let qm_state = warp::any().map(|| Context);
    let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

    let root_node = Arc::new(schema());

    let log = warp::log("warp_subscriptions");

    let routes = warp::path("subscriptions")
        .and(warp::ws())
        .map(serve_schema(root_node))
        .map(websocket_protocol_header)
        .or(warp::post()
            .and(warp::path("graphql"))
            .and(qm_graphql_filter))
        .or(playground())
        .or(homepage())
        .with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
