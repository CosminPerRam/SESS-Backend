
use juniper::graphql_object;
use context::Context;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub async fn hello() -> Vec<String> {
        vec!["hello".to_string()]
    }
}
