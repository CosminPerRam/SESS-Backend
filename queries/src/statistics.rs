use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Statistics {
    queries: i32
}

impl Statistics {
    pub fn from_db(statistics: &statistics::Statistics) -> Self {
        Self {
            queries: statistics.queries as i32
        }
    }
}
