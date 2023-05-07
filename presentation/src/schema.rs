
use juniper::{EmptyMutation, RootNode};
use context::Context;
use queries::Subscription;
use subscriptions::Query;

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), Subscription)
}
