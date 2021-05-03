use chrono::prelude::{DateTime, Utc};

pub trait IDProvider: Send + Sync {
    fn generate(&self) -> String;
}

pub trait TimeProvider: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}
