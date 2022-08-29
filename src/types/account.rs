use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: AccountId,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Default)]
pub struct AccountId(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAccount {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: AccountId,
    // nbf means "not used before timestamp"
    pub nbf: DateTime<Utc>,
}
