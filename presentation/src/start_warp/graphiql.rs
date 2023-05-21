use juniper_warp::graphiql_filter;
use warp::Filter;

pub fn graphiql() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::get()
        .and(warp::path("graphiql"))
        .and(graphiql_filter("/graphql", Some("/subscriptions")))
}
