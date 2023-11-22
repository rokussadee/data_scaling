use actix_web::{web, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path};
use actix::Addr;
use actix::prelude::*;
use chrono::NaiveDate;

use crate::dto::CreateFloraRequest;
use crate::database::PoolType;

use crate::{
    messages::{FetchFlora, CreateFlora},
    AppState, DbActor
};

pub struct FloraService;
pub struct PlantService;

impl FloraService{
    pub async fn get_all() -> HttpResponse {
        HttpResponse::Ok().body("Get all plants.")
    }
    pub async fn post_flora(state: Data<AppState>, path:Path<u64>, body: Json<CreateFloraRequest>) -> impl Responder{
        let id: u64 = path.into_inner();

        let db: Addr<DbActor> = state.as_ref().db.clone();

        match db.send(CreateFlora {
            common_name: body.common_name.to_string(),
            latin_name: body.latin_name.to_string(),
            blossom_season: body.blossom_season.to_string(),
            planting_date: body.planting_date.to_string(),
            date_of_discovery: NaiveDate::parse_from_str( body.planting_date, "%Y-%m-%d"),
            discoverer_id: body.discoverer_id,
            flora_type: body.flora_type,
        }).await
        {
            Ok(Ok(info)) => HttpResponse::Ok().json(info),
            _ => HttpResponse::InternalServerError().json("Failed to create flora."),
        }
    }
    pub async get_flora() {
        
    }
}
