use crate::domain::entities::diagnosis::Diagnosis;
use crate::schema::diagnoses;

#[derive(Queryable, Insertable, Clone)]
#[table_name = "diagnoses"]
pub struct Diagnoses {
    pub id: String,
    pub title: String,
    pub organization_id: String,
}

pub fn to_entity(model: Diagnoses) -> Diagnosis {
    Diagnosis::new(model.id, model.title, model.organization_id)
}
