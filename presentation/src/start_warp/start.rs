use std::env;
use std::sync::Arc;
use warp::Filter;
use crate::schema::schema;
use crate::start_warp::graphiql::graphiql;
use crate::start_warp::graphql::endpoint;
use crate::start_warp::homepage::homepage;
use crate::start_warp::playground::playground;
use crate::start_warp::subscriptions::subscriptions;

pub async fn start_warp() {
    let root_node = Arc::new(schema());

    let log = warp::log("warp_subscriptions");

    let routes = warp::any()
        .and(subscriptions(root_node))
        .or(endpoint(schema()))
        .or(playground())
        .or(graphiql())
        .or(homepage())
        .with(log);

    let server_port: u16 = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080);
    warp::serve(routes).run(([0, 0, 0, 0], server_port)).await;
}
