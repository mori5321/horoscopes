use crate::db::DbPool;
use std::sync::Arc;

pub struct AppState {
    pool: Arc<DbPool>,
}

impl AppState {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> Arc<DbPool> {
        self.pool.clone()
    }
}
