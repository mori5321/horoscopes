use chrono::prelude::{DateTime, Utc};

pub trait IDProvider: Send + Sync {
    fn generate(&self) -> String;
}

pub trait TimeProvider: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

pub trait AccessTokenProvider: Send + Sync {
    fn generate(
        &self,
        user_id: String,
        issued_at_timestamp: u64,
        expires_at_timestamp: u64,
    ) -> String;
    fn verify(&self, access_token: String) -> Result<String, String>;
}
