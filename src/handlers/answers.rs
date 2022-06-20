use std::collections::HashMap;
use warp::{http::StatusCode, reply, Rejection, Reply};

use crate::store;
use crate::types;

pub async fn add_answer_handler(
    params: HashMap<String, String>,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    let answer = types::answers::Answer {
        id: types::answers::AnswerId("1".to_string()),
        question_id: types::questions::QuestionId(params.get("question_id").unwrap().to_string()),
        content: params.get("content").unwrap().to_string(),
    };

    store.answers.write().insert(answer.id.to_string(), answer);

    Ok(reply::with_status("Answer created!", StatusCode::CREATED))
}
