mod statistics;

use juniper::graphql_object;
use context::DatabaseContext;
use crate::statistics::Statistics;

pub struct Query;

#[graphql_object(context = DatabaseContext)]
impl Query {
    pub async fn statistics(&self, context: &DatabaseContext) -> Statistics {
        let DatabaseContext(context) = context;

        let mut context = context.write().await;
        context.statistics.queries = context.statistics.queries + 1;

        Statistics::from_db(&context.statistics)
    }
}
