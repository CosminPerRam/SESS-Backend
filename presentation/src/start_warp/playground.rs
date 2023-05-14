use juniper_warp::playground_filter;
use warp::Filter;
use warp::http::Response;

pub fn playground() -> impl Filter<Extract = (Response<Vec<u8>>,), Error = warp::Rejection> + Clone
{
    warp::get()
        .and(warp::path("playground"))
        .and(playground_filter("/graphql", Some("/subscriptions")))
}
