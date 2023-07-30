pub mod db;

use once_cell::sync::Lazy;
use std::sync::Arc;
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};
use migration::entities::prelude::Query;
use sea_orm::prelude::*;
use crate::db::make_db_connection;

static DATA: Lazy<DatabaseContext> = Lazy::new(|| DatabaseContext(Arc::new(RwLock::new(Database::default()))));

pub struct Database {
    pub connection: DatabaseConnection
}

impl Default for Database {
    fn default() -> Self {
        Database {
            connection: make_db_connection()
        }
    }
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

    pub async fn add_query_made(&self) {
        let context = self.acquire_write_context().await;
        println!("{}", context.connection.ping().await.is_ok());
    }

    pub async fn get_queries_made(&self) -> u64 {
        let mut context = self.acquire_write_context().await;

        Query::find().count(&context.connection).await.unwrap()
    }
}

pub fn get_context() -> DatabaseContext {
    DATA.clone()
}

impl juniper::Context for DatabaseContext {}
