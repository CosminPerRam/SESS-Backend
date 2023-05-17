use juniper_warp::graphiql_filter;
use warp::Filter;
use warp::http::Response;

pub fn graphiql() -> impl Filter<Extract = (Response<Vec<u8>>,), Error = warp::Rejection> + Clone
{
    warp::get()
        .and(warp::path("graphiql"))
        .and(graphiql_filter("/graphql", Some("/subscriptions")))
}
