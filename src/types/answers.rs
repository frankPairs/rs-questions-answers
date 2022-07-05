use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use crate::types::questions::QuestionId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct AnswerId(pub String);

impl Display for AnswerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}", self.0)
    }
}
