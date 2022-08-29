use argon2::{self, Config, Error as Argon2Error};
use chrono::prelude::*;
use paseto::{tokens, PasetoBuilder};
use rand::{thread_rng, Rng};
use serde_json::Value;
use tracing::{event, Level};

const TOKEN_SECRET: &str = "RANDOM WORDS WINTER MACINTOSH PC";

#[derive(Debug)]
pub enum Error {
    EncryptTokenError,
    DecryptTokenError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EncryptTokenError => write!(f, "Cannot encrypt token."),
            Error::DecryptTokenError => write!(f, "Cannot decrypt token."),
        }
    }
}

///
/// It creates a hash based on a password using [the Argon2 alghorithm](https://en.wikipedia.org/wiki/Argon2)
///
pub fn hash_password(password: &[u8]) -> String {
    let salt = thread_rng().gen::<[u8; 32]>();
    let config = Config::default();

    argon2::hash_encoded(password, &salt, &config).unwrap()
}

///
/// It verifies if a password equals to an encoded password (using the hash_password method).
///
/// Returns true if password was verified successfully.
///
/// ## Example
///
/// ```
/// let pwd = "test1234";
/// let encoded_pwd = hash_password(pwd.as_bytes());
///
/// assert!(verify_password(encoded_pwd, pwd.as_bytes()).unwrap());
/// ```
///
pub fn verify_password(encoded_password: &str, password: &[u8]) -> Result<bool, Argon2Error> {
    argon2::verify_encoded(encoded_password, password)
}

pub fn encode_token(
    claim_key: String,
    claim_value: serde_json::value::Value,
) -> Result<String, Error> {
    let current_date = Utc::now();
    let one_day_duration = current_date + chrono::Duration::days(1);

    PasetoBuilder::new()
        .set_encryption_key(&Vec::from(TOKEN_SECRET.as_bytes()))
        .set_expiration(&one_day_duration)
        .set_not_before(&current_date)
        .set_claim(&claim_key, claim_value)
        .build()
        .map_err(|err| {
            event!(Level::ERROR, "{}", err);

            Error::EncryptTokenError
        })
}

pub fn decode_token(token: String) -> Result<Value, Error> {
    tokens::validate_local_token(
        &token,
        None,
        TOKEN_SECRET.as_bytes(),
        &tokens::TimeBackend::Chrono,
    )
    .map_err(|err| {
        event!(Level::ERROR, "{}", err);

        Error::DecryptTokenError
    })
}
