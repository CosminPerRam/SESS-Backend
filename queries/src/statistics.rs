use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Statistics {
    statistics_queries: i32,
    servers_queries: i32
}

impl Statistics {
    pub fn from_db(statistics: &statistics::Statistics) -> Self {
        Self {
            statistics_queries: statistics.statistics_queries as i32,
            servers_queries: statistics.servers_queries as i32
        }
    }
}
