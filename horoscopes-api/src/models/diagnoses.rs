use crate::schema::diagnoses;

#[derive(Queryable, Insertable, Clone)]
#[table_name = "diagnoses"]
pub struct Diagnoses {
    pub id: String,
    pub title: String,
    pub organization_id: String,
}
