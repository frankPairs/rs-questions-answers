use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as ReqwestMiddlewareError;
use std::fmt::{Display, Formatter};
use tracing::{event, Level};
use warp::reject::Reject;
use warp::{
    cors::CorsForbidden, filters::body::BodyDeserializeError, http::StatusCode, reply, Rejection,
    Reply,
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    BadQuestionId,
    MissingParameters,
    QuestionNotFound,
    DatabaseQueryError,
    ClientError(ApiLayerError),
    ServerError(ApiLayerError),
    ReqwestAPIError(ReqwestError),
    MiddlewareReqwestAPIError(ReqwestMiddlewareError),
}

#[derive(Debug, Clone)]
pub struct ApiLayerError {
    pub status: u16,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => {
                write!(f, "Missing parameters")
            }
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::BadQuestionId => write!(f, "Question id must be an integer"),
            Error::DatabaseQueryError => write!(f, "Database error"),
            Error::ReqwestAPIError(err) => write!(f, "External API error: {}", err),
            Error::MiddlewareReqwestAPIError(err) => write!(f, "External API error: {}", err),
            Error::ClientError(err) => write!(f, "External client error: {}", err),
            Error::ServerError(err) => write!(f, "External server error: {}", err),
        }
    }
}

impl Reject for Error {}

impl Display for ApiLayerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl Reject for ApiLayerError {}

pub async fn error_handler(rej: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if rej.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(err) = rej.find::<CorsForbidden>() {
        event!(Level::ERROR, "{}", err);

        Ok(reply::with_status(
            "Cors Forbidden error",
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(err) = rej.find::<BodyDeserializeError>() {
        event!(Level::ERROR, "{}", err);

        Ok(reply::with_status(
            "Bad parameters",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::DatabaseQueryError) = rej.find() {
        Ok(reply::with_status(
            "Database error",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::ReqwestAPIError(err)) = rej.find() {
        event!(Level::ERROR, "{}", err);

        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::MiddlewareReqwestAPIError(err)) = rej.find() {
        event!(Level::ERROR, "{}", err);

        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ClientError(err)) = rej.find() {
        event!(Level::ERROR, "{}", err);

        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ServerError(err)) = rej.find() {
        event!(Level::ERROR, "{}", err);

        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
