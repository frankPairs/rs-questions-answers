use argon2::{self, Config, Error as Argon2Error};
use chrono::prelude::*;
use paseto::PasetoBuilder;
use rand::{thread_rng, Rng};

const TOKEN_SECRET: &str = "RANDOM WORDS WINTER MACINTOSH PC";

#[derive(Debug)]
pub enum Error {
    EncryptTokenError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EncryptTokenError => write!(f, "Encrypt token error."),
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

pub fn gen_token(
    claim_key: String,
    claim_value: serde_json::value::Value,
) -> Result<String, Error> {
    let current_date = Utc::now();
    let one_day_duration = current_date + chrono::Duration::days(1);

    PasetoBuilder::new()
        .set_encryption_key(TOKEN_SECRET.as_bytes())
        .set_not_before(&one_day_duration)
        .set_claim(&claim_key, claim_value)
        .build()
        .map_err(|_| Error::EncryptTokenError)
}
