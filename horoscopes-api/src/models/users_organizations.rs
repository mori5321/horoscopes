use crate::schema::users_organizations;

#[derive(Queryable, Insertable, Clone)]
#[table_name = "users_organizations"]
pub struct UsersOrganizations {
    pub organization_id: String,
    pub user_id: String,
}
