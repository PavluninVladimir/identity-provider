use actix_web::{post, web, HttpResponse, Responder};
use crate::{models::{RegisterRequest, LoginRequest, User}, token, AppState};

#[post("/auth/register")]
pub async fn register(state: web::Data<AppState>, payload: web::Json<RegisterRequest>) -> impl Responder {
    let req = payload.into_inner();
    match User::create(&state.db, req.email, req.password).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => {
            eprintln!("register error: {err:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/auth/login")]
pub async fn login(state: web::Data<AppState>, payload: web::Json<LoginRequest>) -> impl Responder {
    let req = payload.into_inner();
    match User::find_by_email(&state.db, &req.email).await {
        Ok(Some(user)) if user.verify_password(&req.password) => {
            let token = token::generate(&user.id, &state.jwt_secret);
            HttpResponse::Ok().json(serde_json::json!({"token": token}))
        }
        _ => HttpResponse::Unauthorized().finish(),
    }
}
