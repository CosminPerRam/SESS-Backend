use std::sync::Arc;
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::subscriptions::serve_graphql_ws;
use warp::{Filter, Reply};
use warp::reply::WithHeader;
use warp::ws::Ws;
use futures::{FutureExt as _};
use context::{DatabaseContext, get_context};
use crate::schema::Schema;

pub fn websocket_protocol_header(reply: impl Reply + Sized) -> WithHeader<impl Reply + Sized + Sized> {
    warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
}

pub fn serve_schema(root_node: Arc<Schema>) -> impl Fn(Ws) -> Box<dyn Reply> + Clone
{
    move |ws: Ws| {
        let root_node = root_node.clone();
        Box::new(ws.on_upgrade(move |websocket| async move {
            serve_graphql_ws(websocket, root_node, ConnectionConfig::new(get_context()))
                .map(|r| {
                    if let Err(e) = r {
                        println!("Websocket error: {e}");
                    }
                })
                .await
        }))
    }
}

pub fn endpoint(schema: Schema) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let qm_schema = schema;
    let qm_state = warp::any().map(|| get_context());
    let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

    warp::post()
        .and(warp::path("graphql"))
        .and(qm_graphql_filter)
}
