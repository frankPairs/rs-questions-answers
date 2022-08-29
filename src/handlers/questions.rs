use std::collections::HashMap;
use tracing::event;
use tracing::Level;
use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use crate::profanity;
use crate::store;
use crate::types::{
    account::Session,
    pagination::{extract_pagination, Pagination},
    questions::{NewQuestion, Question},
};

pub async fn get_questions_handler(
    params: HashMap<String, String>,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(tracing::Level::INFO, pagination = true);

        pagination = extract_pagination(params)?;
    }

    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(reply::json(&res)),
        Err(err) => Err(reject::custom(err)),
    }
}

/// Add a new question coming from a POST request to the database.
///
/// It checks if title or the content of the question contain a censored content. We use tokio::spawn for executing the different
/// calls to the profanity service in parallel.
pub async fn add_question_handler(
    session: Session,
    new_question: NewQuestion,
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
        account_id: session.account_id,
    };

    match store.add_question(question).await {
        Ok(question) => Ok(reply::json(&question)),
        Err(err) => {
            event!(Level::ERROR, "{}", err);

            Err(reject::custom(err))
        }
    }
}

pub async fn get_question_handler(
    question_id: i32,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    match store.get_question(question_id).await {
        Ok(question) => Ok(reply::json(&question)),
        Err(err) => Err(reject::custom(err)),
    }
}

pub async fn delete_question_handler(
    question_id: i32,
    session: Session,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    let is_question_owner = store
        .is_question_owner(question_id, session.account_id.0)
        .await?;

    if !is_question_owner {
        return Err(reject::custom(handle_errors::Error::Unauthorized));
    }

    match store.delete_question(question_id).await {
        Ok(_) => Ok(reply::with_status("Question deleted!", StatusCode::OK)),
        Err(err) => Err(reject::custom(err)),
    }
}

/// Update an existiong question coming from a PUT request.
///
/// It checks if title or the content of the question contain a censored content. We use tokio::join for executing the different
/// calls to the profanity service concurrenty.
pub async fn update_question_handler(
    question_id: i32,
    session: Session,
    question: Question,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    let is_question_owner = store
        .is_question_owner(question_id, session.account_id.0)
        .await?;

    if !is_question_owner {
        return Err(reject::custom(handle_errors::Error::Unauthorized));
    }

    let title_task = profanity::check_profanity(question.title);
    let content_task = profanity::check_profanity(question.content);
    let (title_res, content_res) = tokio::join!(title_task, content_task);

    if title_res.is_err() {
        return Err(reject::custom(title_res.unwrap_err()));
    };

    if content_res.is_err() {
        return Err(reject::custom(content_res.unwrap_err()));
    };

    let question_updated = Question {
        id: question.id,
        title: title_res.unwrap(),
        content: content_res.unwrap(),
        tags: question.tags,
    };

    match store.update_question(question_updated, question_id).await {
        Ok(_) => Ok(reply::with_status("Question updated!", StatusCode::OK)),
        Err(err) => Err(reject::custom(err)),
    }
}
