use std::collections::HashMap;
use tracing::event;
use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use crate::store;
use crate::types;

pub async fn get_questions_handler(
    params: HashMap<String, String>,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    let mut pagination = types::pagination::Pagination::default();

    if !params.is_empty() {
        event!(tracing::Level::INFO, pagination = true);

        pagination = types::pagination::extract_pagination(params)?;
    }

    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(reply::json(&res)),
        Err(err) => Err(reject::custom(err)),
    }
}

pub async fn add_question_handler(
    new_question: types::questions::NewQuestion,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    match store.add_question(new_question).await {
        Ok(_) => Ok(reply::with_status("Question created!", StatusCode::CREATED)),
        Err(err) => Err(reject::custom(err)),
    }
}

pub async fn get_question_handler(
    question_id: u32,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    match store.get_question(question_id).await {
        Ok(question) => Ok(reply::json(&question)),
        Err(err) => Err(reject::custom(err)),
    }
}

pub async fn delete_question_handler(
    question_id: u32,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    match store.delete_question(question_id).await {
        Ok(_) => Ok(reply::with_status("Question deleted!", StatusCode::OK)),
        Err(err) => Err(reject::custom(err)),
    }
}

pub async fn update_question_handler(
    question_id: u32,
    question: types::questions::Question,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    match store.update_question(question, question_id).await {
        Ok(_) => Ok(reply::with_status("Question updated!", StatusCode::OK)),
        Err(err) => Err(reject::custom(err)),
    }
}
