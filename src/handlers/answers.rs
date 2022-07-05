use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use crate::store;
use crate::types;

pub async fn add_answer_handler(
    new_answer: types::answers::NewAnswer,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok(reply::with_status("Answer created!", StatusCode::CREATED)),
        Err(err) => Err(reject::custom(err)),
    }
}
