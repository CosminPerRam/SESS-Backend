use std::env;
use warp::Filter;
use warp::http::Method;
use crate::start_warp::graphiql::graphiql;
use crate::start_warp::graphql::graphql;
use crate::start_warp::homepage::homepage;
use crate::start_warp::playground::playground;
use crate::start_warp::subscriptions::subscriptions;

pub async fn start_warp() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(&[Method::POST, Method::OPTIONS]);

    let routes = subscriptions()
        .or(graphql())
        .or(homepage())
        .or(playground())
        .or(graphiql())
        .with(cors);

    let server_port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    if cfg!(debug_assertions) {
        warp::serve(routes).run(([0, 0, 0, 0], server_port)).await;
    } else {
        warp::serve(routes)
            .tls()
            .cert_path(env::var("CERT_PATH").unwrap_or_else(|_| "cert.pem".to_string()))
            .key_path(env::var("KEY_PATH").unwrap_or_else(|_| "privkey.pem".to_string()))
            .run(([0, 0, 0, 0], server_port)).await;
    }
}
