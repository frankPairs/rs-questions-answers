use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::event;
use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use crate::store;
use crate::types;
use crate::types::bad_words::BadWordsResponse;
use crate::types::questions::NewQuestion;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

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
    let client = reqwest::Client::new();
    let bad_words_response = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "H1WemIqKY1WNOTICqeIWV3YgH7XgwTrY")
        .body(new_question.content)
        .send()
        .await
        .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
    let bad_words_status = bad_words_response.status();

    if !bad_words_status.is_success() {
        let err = transform_error(bad_words_response).await;

        if bad_words_status.is_client_error() {
            return Err(reject::custom(handle_errors::Error::ClientError(err)));
        }

        return Err(reject::custom(handle_errors::Error::ServerError(err)));
    }

    let json_response = bad_words_response
        .json::<BadWordsResponse>()
        .await
        .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
    let content = json_response.censored_content;
    let question = NewQuestion {
        title: new_question.title,
        content,
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
    match store.update_question(question, question_id).await {
        Ok(_) => Ok(reply::with_status("Question updated!", StatusCode::OK)),
        Err(err) => Err(reject::custom(err)),
    }
}

async fn transform_error(res: reqwest::Response) -> handle_errors::ApiLayerError {
    handle_errors::ApiLayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}
