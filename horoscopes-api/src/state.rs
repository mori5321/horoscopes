use crate::db::MysqlPool;
use std::sync::Arc;

pub struct AppState {
    pool: Arc<MysqlPool>,
}

impl AppState {
    pub fn new(pool: Arc<MysqlPool>) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> Arc<MysqlPool> {
        self.pool.clone()
    }
}
