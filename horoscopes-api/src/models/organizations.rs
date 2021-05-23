use crate::schema::organizations;

#[derive(Queryable, Insertable, Clone)]
#[table_name = "organizations"]
pub struct Organizations {
    pub id: String,
    pub name: String,
}
