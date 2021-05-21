use crate::schema::accounts;

#[derive(Queryable, Insertable, Clone)]
#[table_name = "accounts"]
pub struct Accounts {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub user_id: String,
}
