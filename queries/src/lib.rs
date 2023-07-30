mod statistics;

use juniper::graphql_object;
use context::DatabaseContext;
use crate::statistics::Statistics;

pub struct Query;

#[graphql_object(context = DatabaseContext)]
impl Query {
    pub async fn statistics(&self, context: &DatabaseContext) -> Statistics {
        Statistics {
            master_queries: context.get_queries_made().await as i32
        }
    }
}
