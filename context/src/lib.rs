use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use statistics::Statistics;

static DATA: Lazy<DatabaseContext> = Lazy::new(|| {
    let stats = Statistics {
        queries: 0
    };

    let database = Database {
        statistics: stats
    };

    DatabaseContext(Arc::new(RwLock::new(database)))
});

pub struct Database {
    pub statistics: Statistics
}

#[derive(Clone)]
pub struct DatabaseContext(pub Arc<RwLock<Database>>);

pub fn get_context() -> DatabaseContext {
    DATA.clone()
}

impl juniper::Context for DatabaseContext {}
