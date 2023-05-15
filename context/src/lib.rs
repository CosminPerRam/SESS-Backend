use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};
use statistics::Statistics;

static DATA: Lazy<DatabaseContext> = Lazy::new(|| DatabaseContext(Arc::new(RwLock::new(Database::default()))));

#[derive(Default)]
pub struct Database {
    pub statistics: Statistics
}

#[derive(Clone)]
pub struct DatabaseContext(pub Arc<RwLock<Database>>);

impl DatabaseContext {
    async fn acquire_write_context(&self) -> RwLockWriteGuard<'_, Database> {
        self.0.write().await
    }

    async fn acquire_read_context(&self) -> RwLockReadGuard<'_, Database> {
        self.0.read().await
    }

    pub async fn get_statistics(&self) -> Statistics {
        self.acquire_read_context().await.statistics.clone()
    }

    pub async fn add_server_query_visit(&self) {
        let mut context = self.acquire_write_context().await;
        context.statistics.servers_queries += 1;
    }

    pub async fn add_statistics_query_visit(&self) {
        let mut context = self.acquire_write_context().await;
        context.statistics.statistics_queries += 1;
    }
}

pub fn get_context() -> DatabaseContext {
    DATA.clone()
}

impl juniper::Context for DatabaseContext {}
