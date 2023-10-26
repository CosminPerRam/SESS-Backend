use std::sync::Arc;
use std::time::Duration;
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::subscriptions::serve_graphql_ws;
use warp::{Filter, Rejection, Reply};
use warp::reply::WithHeader;
use warp::ws::Ws;
use futures::{FutureExt as _};
use context::get_context;
use crate::schema::{Schema, schema};

pub fn serve_schema(root_node: Arc<Schema>) -> impl Fn(Ws) -> Box<dyn Reply> + Clone
{
    move |ws: Ws| {
        let root_node = root_node.clone();
        Box::new(ws.on_upgrade(move |websocket| async move {
            let config = ConnectionConfig::new(get_context());
            let config = config.with_keep_alive_interval(Duration::from_secs(2));

            serve_graphql_ws(websocket, root_node, config)
                .map(|r| {
                    if let Err(e) = r {
                        println!("Websocket error: {e}");
                    }
                })
                .await
        }))
    }
}

pub fn websocket_protocol_header(reply: impl Reply + Sized) -> WithHeader<impl Reply + Sized + Sized> {
    warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
}

pub fn subscriptions() -> impl Filter + Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let log = warp::log("warp_subscriptions");

    let root_node = Arc::new(schema());

    warp::path("subscriptions")
        .and(warp::ws())
        .map(serve_schema(root_node))
        .map(websocket_protocol_header)
        .with(log)
}
