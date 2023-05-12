use tokio::sync::RwLock;
use statistics::Statistics;

#[derive(Clone)]
pub struct Database {
    pub statistics: Statistics
}

pub struct DatabaseContext(pub RwLock<Database>);

pub fn get_context() -> DatabaseContext {
    let stats = Statistics {
        queries: 0
    };

    let database = Database {
        statistics: stats
    };

    DatabaseContext(RwLock::new(database))
}

impl juniper::Context for DatabaseContext {}
