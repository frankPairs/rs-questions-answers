use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use crate::profanity;
use crate::store;
use crate::types::answers::NewAnswer;

pub async fn add_answer_handler(
    new_answer: NewAnswer,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    let content = match profanity::check_profanity(new_answer.content).await {
        Ok(censored_content) => censored_content,
        Err(err) => return Err(reject::custom(err)),
    };
    let answer_updated = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };

    match store.add_answer(answer_updated).await {
        Ok(_) => Ok(reply::with_status("Answer created!", StatusCode::CREATED)),
        Err(err) => Err(reject::custom(err)),
    }
}
