use super::diagnosis::ID as DiagnosisID;

use answer_frame::AnswerFrame;

#[derive(Clone, Debug)]
pub struct Question {
    id: ID,
    text: Text,
    diagnosis_id: DiagnosisID,
    answer_frames: Vec<AnswerFrame>,
}

impl Question {
    pub fn add_answer_frame(&mut self, answer_frame: AnswerFrame) {
        self.answer_frames.push(answer_frame)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ID(String);

impl ID {
    pub fn new(id: String) -> Self {
        ID(id)
    }

    pub fn value(&self) -> String {
        let ID(id) = self;
        id.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Text(String);

impl Text {
    pub fn new(text: String) -> Self {
        Text(text)
    }

    pub fn value(&self) -> String {
        let Text(text) = self;
        text.clone()
    }
}

mod answer_frame {
    use super::ID as QuestionID;

    #[derive(Clone, Debug)]
    pub struct AnswerFrame {
        id: ID,
        text: Text,
        question_id: QuestionID,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct ID(String);

    #[derive(Clone, Debug)]
    struct Text(String);
}
