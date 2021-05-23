use crate::schema::answer_frames;

#[derive(Queryable, Insertable, Clone)]
#[table_name = "answer_frames"]
pub struct AnswerFrames {
    pub id: String,
    pub text: String,
    pub question_id: String,
}
