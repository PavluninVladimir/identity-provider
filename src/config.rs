use std::env;
use dotenvy::dotenv;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub mongodb_uri: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT").unwrap_or_else(|_| "8080".into()).parse().unwrap(),
            mongodb_uri: env::var("MONGODB_URI").expect("MONGODB_URI must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}
