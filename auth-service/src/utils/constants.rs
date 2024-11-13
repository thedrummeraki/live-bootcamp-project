use std::env as std_env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_database_url();
}

pub const JWT_COOKIE_NAME: &str = "jwt";

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL: &str = "DATABASE_URL";
}

fn set_token() -> String {
    dotenvy::dotenv().ok();
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

fn set_database_url() -> String {
    dotenvy::dotenv().ok();
    let database_url = std_env::var(env::DATABASE_URL).expect("DATABASE_URL not set");
    if database_url.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }
    database_url
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
