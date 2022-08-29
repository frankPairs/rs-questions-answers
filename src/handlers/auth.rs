use std::future;
use tracing::{event, Level};
use warp::{header, reject, reply, Filter, Rejection, Reply};

use crate::crypt::{decode_token, encode_token, verify_password};
use crate::store::Store;
use crate::types::account::{Credentials, NewAccount, Session};

/// Handler responsible to register a new account to the database.
pub async fn register(new_account: NewAccount, store: Store) -> Result<impl Reply, Rejection> {
    match store.add_account(new_account).await {
        Ok(account) => Ok(reply::json(&account)),
        Err(err) => Err(reject::custom(err)),
    }
}

pub async fn login(credentials: Credentials, store: Store) -> Result<impl Reply, Rejection> {
    match store.get_account(credentials.email).await {
        Ok(account) => match verify_password(&account.password, credentials.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    let token_result =
                        encode_token(String::from("account_id"), serde_json::json!(account.id.0));

                    match token_result {
                        Ok(token) => Ok(reply::json(&token)),
                        Err(_) => Err(reject::custom(handle_errors::Error::TokenError)),
                    }
                } else {
                    Err(reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(err) => Err(reject::custom(handle_errors::Error::ArgonLibraryError(err))),
        },
        Err(err) => Err(reject::custom(err)),
    }
}

pub fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    match decode_token(token) {
        Ok(value) => serde_json::from_value::<Session>(value).map_err(|err| {
            event!(Level::ERROR, "{}", err);

            handle_errors::Error::TokenError
        }),
        Err(err) => {
            event!(Level::ERROR, "{}", err);

            Err(handle_errors::Error::TokenError)
        }
    }
}

pub fn auth() -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    header::<String>("Authorization").and_then(|token: String| {
        let session_result = verify_token(token);
        let session = match session_result {
            Ok(session) => session,
            Err(err) => {
                event!(Level::ERROR, "{}", err);

                return future::ready(Err(reject::custom(handle_errors::Error::TokenError)));
            }
        };

        future::ready(Ok(session))
    })
}
