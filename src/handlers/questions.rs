use std::collections::HashMap;
use tracing::event;
use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use crate::profanity;
use crate::store;
use crate::types;
use crate::types::questions::{NewQuestion, Question};

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
    let get_title_task = tokio::spawn(profanity::check_profanity(new_question.title));
    let get_content_task = tokio::spawn(profanity::check_profanity(new_question.content));
    let (title_res, content_res) = (
        get_title_task.await.unwrap(),
        get_content_task.await.unwrap(),
    );

    if title_res.is_err() {
        return Err(reject::custom(title_res.unwrap_err()));
    };

    if content_res.is_err() {
        return Err(reject::custom(content_res.unwrap_err()));
    };

    let question = NewQuestion {
        title: title_res.unwrap(),
        content: content_res.unwrap(),
        tags: new_question.tags,
    };

    match store.add_question(question).await {
        Ok(question) => Ok(reply::json(&question)),
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
    let title = match profanity::check_profanity(question.title).await {
        Ok(censored_title) => censored_title,
        Err(err) => return Err(reject::custom(err)),
    };
    let content = match profanity::check_profanity(question.content).await {
        Ok(censored_content) => censored_content,
        Err(err) => return Err(reject::custom(err)),
    };
    let question_updated = Question {
        id: question.id,
        title,
        content,
        tags: question.tags,
    };

    match store.update_question(question_updated, question_id).await {
        Ok(_) => Ok(reply::with_status("Question updated!", StatusCode::OK)),
        Err(err) => Err(reject::custom(err)),
    }
}
