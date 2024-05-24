use crate::services::list_service;
use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/test")]
async fn test(app_state: web::Data<AppState>) -> impl Responder {
    match list_service::create("test".to_string(), &app_state.conn).await {
        Ok(model) => HttpResponse::Ok().json(model),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
