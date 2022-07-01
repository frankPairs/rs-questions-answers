use handle_errors::Error;
use std::collections::HashMap;
use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use crate::store;
use crate::types;

pub async fn add_answer_handler(
    params: HashMap<String, String>,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    let parse_question_id_param_result = params.get("question_id").unwrap().parse::<i32>();

    match parse_question_id_param_result {
        Ok(question_id) => {
            let answer = types::answers::Answer {
                id: types::answers::AnswerId("1".to_string()),
                question_id: types::questions::QuestionId(question_id),
                content: params.get("content").unwrap().to_string(),
            };

            store.answers.write().insert(answer.id.to_string(), answer);

            Ok(reply::with_status("Answer created!", StatusCode::CREATED))
        }
        Err(_) => Err(reject::custom(Error::BadQuestionId)),
    }
}
