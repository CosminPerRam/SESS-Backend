use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Statistics {
    pub master_queries: i32,
}
