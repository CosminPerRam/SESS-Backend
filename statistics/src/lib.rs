
#[derive(Clone, Default)]
pub struct Statistics {
    pub statistics_queries: u32,
    pub servers_queries: u32
}

impl Statistics {
    pub fn add_statistics_query_visit(&mut self) {
        self.statistics_queries += 1;
    }

    pub fn add_servers_query_visit(&mut self) {
        self.servers_queries += 1;
    }
}
