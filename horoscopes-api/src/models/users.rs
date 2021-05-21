use crate::schema::users;

#[derive(Queryable, Insertable, Clone)]
#[table_name = "users"]
pub struct Users {
    pub id: String,
}
