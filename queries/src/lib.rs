mod statistics;

use juniper::graphql_object;
use context::DatabaseContext;
use crate::statistics::Statistics;

pub struct Query;

#[graphql_object(context = DatabaseContext)]
impl Query {
    pub async fn statistics(&self, context: &DatabaseContext) -> Statistics {
        context.add_statistics_query_visit().await;

        Statistics::from_db(&context.get_statistics().await)
    }
}
