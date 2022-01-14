use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Houston, we have a lift off!".to_string())
}
