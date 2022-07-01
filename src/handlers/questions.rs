use handle_errors::Error;
use std::collections::HashMap;
use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use crate::store;
use crate::types;

pub async fn get_questions_handler(
    params: HashMap<String, String>,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    let pagination = types::pagination::extract_pagination(params)?;
    let questions: Vec<types::questions::Question> =
        store.questions.read().values().cloned().collect();
    let questions = &questions[pagination.start..pagination.end];

    Ok(reply::json(&questions))
}

pub async fn add_question_handler(
    new_question: types::questions::Question,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .insert(new_question.id.to_string(), new_question);

    Ok(reply::with_status("Question created!", StatusCode::CREATED))
}

pub async fn get_question_handler(
    question_id: i32,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write()
        .get(&types::questions::QuestionId(question_id).to_string())
    {
        Some(question) => Ok(reply::json(question)),
        None => Err(reject::custom(Error::QuestionNotFound)),
    }
}

pub async fn delete_question_handler(
    question_id: i32,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write()
        .remove(&types::questions::QuestionId(question_id).to_string())
    {
        Some(_) => Ok(reply::with_status("Question deleted!", StatusCode::OK)),
        None => Err(reject::custom(Error::QuestionNotFound)),
    }
}

pub async fn update_question_handler(
    question_id: i32,
    question: types::questions::Question,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write()
        .get_mut(&types::questions::QuestionId(question_id).to_string())
    {
        Some(q) => *q = question,
        None => return Err(reject::custom(Error::QuestionNotFound)),
    }

    Ok(reply::with_status("Question updated!", StatusCode::OK))
}
