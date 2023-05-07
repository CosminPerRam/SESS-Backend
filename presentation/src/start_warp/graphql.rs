use std::sync::Arc;
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::subscriptions::serve_graphql_ws;
use warp::Reply;
use warp::reply::WithHeader;
use warp::ws::Ws;
use futures::{FutureExt as _};
use crate::schema::{Context, Schema};

pub fn websocket_protocol_header(reply: impl Reply + Sized) -> WithHeader<impl Reply + Sized + Sized> {
    warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
}

pub fn serve_schema(root_node: Arc<Schema>) -> impl Fn(Ws) -> Box<dyn Reply> + Clone
{
    move |ws: Ws| {
        let root_node = root_node.clone();
        Box::new(ws.on_upgrade(move |websocket| async move {
            serve_graphql_ws(websocket, root_node, ConnectionConfig::new(Context))
                .map(|r| {
                    if let Err(e) = r {
                        println!("Websocket error: {e}");
                    }
                })
                .await
        }))
    }
}
