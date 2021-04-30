#[derive(Queryable, Clone)]
pub struct Accounts {
    pub id: String,
    pub email: String,
    pub password_hash: String,
}
