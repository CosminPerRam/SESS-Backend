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

#[derive(Clone)]
pub struct Database {
    pub statistics: Statistics
}

pub struct DatabaseContext(pub Arc<RwLock<Database>>);

pub fn get_context() -> DatabaseContext {
    DatabaseContext(DATA.0.clone())
}

impl juniper::Context for DatabaseContext {}
