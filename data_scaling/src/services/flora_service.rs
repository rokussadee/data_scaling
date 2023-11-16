use actix_web::{web, HttpResponse};

pub struct FloraService;
pub struct PlantService;

impl FloraService{
    pub async fn get_all() -> HttpResponse {
        HttpResponse::Ok().body("Get all plants.")
    }
}
