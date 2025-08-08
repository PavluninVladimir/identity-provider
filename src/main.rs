mod config;
mod db;
mod models;
mod routes;
mod token;

use actix_web::{App, HttpServer, web};
use config::Config;
use db::init_db;
use mongodb::Database;

pub struct AppState {
    pub db: Database,
    pub jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = Config::from_env();
    let db = init_db(&cfg.mongodb_uri).await.expect("failed to connect to db");
    let state = web::Data::new(AppState {
        db,
        jwt_secret: cfg.jwt_secret.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(routes::auth::register)
            .service(routes::auth::login)
    })
    .bind((cfg.host, cfg.port))?
    .run()
    .await
}
