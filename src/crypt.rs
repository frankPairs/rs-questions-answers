use argon2::{self, Config};
use rand::{thread_rng, Rng};

/// It creates a hash based on a password using [the Argon2 alghorithm](https://en.wikipedia.org/wiki/Argon2)
pub fn hash_password(password: &[u8]) -> String {
    let salt = thread_rng().gen::<[u8; 32]>();
    let config = Config::default();

    argon2::hash_encoded(password, &salt, &config).unwrap()
}
