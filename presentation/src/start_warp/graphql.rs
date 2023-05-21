use warp::Filter;
use warp::http::Response;
use context::get_context;
use crate::schema::Schema;

pub fn endpoint(schema: Schema) -> impl Filter<Extract = (Response<Vec<u8>>,), Error = warp::Rejection> + Clone {
    let qm_schema = schema;
    let qm_state = warp::any().map(get_context);
    let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

    warp::post()
        .and(warp::path("graphql"))
        .and(qm_graphql_filter)
}
