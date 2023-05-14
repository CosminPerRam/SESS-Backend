use warp::Filter;
use warp::http::Response;

pub fn homepage() -> impl Filter<Extract = (Result<Response<&'static str>, warp::http::Error>,), Error = warp::Rejection> + Clone
{
    warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body("<html><h1>juniper_subscriptions demo</h1><div>visit <a href=\"/playground\">graphql playground</a></html>")
    })
}
