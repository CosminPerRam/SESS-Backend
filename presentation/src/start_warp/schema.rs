use warp::Reply;
use warp::reply::WithHeader;

pub fn grahpql_websocket_header(reply: impl Reply + Sized) -> WithHeader<impl Reply + Sized + Sized> {
    warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
}
