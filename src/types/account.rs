use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: AccountId,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct AccountId(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAccount {
    pub email: String,
    pub password: String,
}
