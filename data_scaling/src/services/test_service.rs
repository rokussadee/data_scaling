
use actix_web::{web, HttpResponse};

pub struct TestService;

impl TestService{
    pub async fn test() -> HttpResponse {
        HttpResponse::Ok().body("Test message.")
    }
}
