use warp::{reject, reply, Rejection, Reply};

use crate::store::Store;
use crate::types::account::NewAccount;

/// Handler responsible to register a new account to the database.
pub async fn register(new_account: NewAccount, store: Store) -> Result<impl Reply, Rejection> {
    match store.add_account(new_account).await {
        Ok(account) => Ok(reply::json(&account)),
        Err(err) => Err(reject::custom(err)),
    }
}
