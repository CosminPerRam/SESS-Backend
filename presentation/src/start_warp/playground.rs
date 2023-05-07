use juniper_warp::playground_filter;
use warp::Filter;

pub fn playground() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::get()
        .and(warp::path("playground"))
        .and(playground_filter("/graphql", Some("/subscriptions")))
}
