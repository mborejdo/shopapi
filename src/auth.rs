use argon2rs::argon2i_simple;
use std::env;

pub fn hash(password: &str) -> String {
    let salt = env::var("AUTH_SALT").expect("environment variable: AUTH_SALT");
    argon2i_simple(&password, &salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}
