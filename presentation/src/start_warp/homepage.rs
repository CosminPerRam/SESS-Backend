use warp::Filter;
use warp::http::Response;

pub fn homepage() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body("<html><h1>juniper_subscriptions demo</h1><div>visit <a href=\"/playground\">graphql playground</a></html>")
    })
}
