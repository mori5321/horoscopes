use crate::usecases::common::ports::providers::TimeProvider;
use chrono::prelude::{DateTime, Utc};

pub struct UTCTimeProvider {}

impl UTCTimeProvider {}

impl UTCTimeProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl TimeProvider for UTCTimeProvider {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
