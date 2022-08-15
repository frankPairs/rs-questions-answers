use warp::{reject, reply, Rejection, Reply};

use crate::crypt::{gen_token, verify_password};
use crate::store::Store;
use crate::types::account::{Credentials, NewAccount};

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
                    let token_result = gen_token(account.email);

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
