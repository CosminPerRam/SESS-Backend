
use juniper::{EmptyMutation, RootNode};
use context::DatabaseContext;
use queries::Query;
use subscriptions::Subscription;

pub type Schema = RootNode<'static, Query, EmptyMutation<DatabaseContext>, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), Subscription)
}
