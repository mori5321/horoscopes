use crate::schema::questions;

#[derive(Queryable, Insertable, Clone)]
#[table_name = "questions"]
pub struct Questions {
    pub id: String,
    pub text: String,
    pub diagnosis_id: String,
    pub question_type: i32,
}
