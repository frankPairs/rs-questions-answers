use std::fmt::{Display, Formatter};
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
        }
    }
}

impl Reject for Error {}

pub async fn error_handler(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(Error::DatabaseQueryError) = err.find() {
        Ok(reply::with_status(
            "Database error",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(_err) = err.find::<CorsForbidden>() {
        Ok(reply::with_status(
            "Cors Forbidden error",
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(_err) = err.find::<BodyDeserializeError>() {
        Ok(reply::with_status(
            "Bad parameters",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(_err) = err.find::<Error>() {
        Ok(reply::with_status(
            "Query parameters error",
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else {
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
