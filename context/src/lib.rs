use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::{RwLock};

static DATA: Lazy<DatabaseContext> = Lazy::new(|| DatabaseContext(Arc::new(RwLock::new(Database::default()))));

#[derive(Default)]
pub struct Database;

#[derive(Clone)]
pub struct DatabaseContext(pub Arc<RwLock<Database>>);

impl DatabaseContext {

}

pub fn get_context() -> DatabaseContext {
    DATA.clone()
}

impl juniper::Context for DatabaseContext {}
