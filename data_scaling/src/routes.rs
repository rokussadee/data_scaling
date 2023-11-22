use actix_web::web::ServiceConfig;
use actix_web::{web, Scope};
use crate::services::flora_service::{FloraService, PlantService};
use crate::services::test_service::{TestService};

pub fn configure_routes(cfg: &mut web::ServiceConfig)  {
    cfg.service(
        web::scope("/flora")
            .service(web::resource("/").to(FloraService::get_all))
            .service(web::resource("/create").to(FloraService::post_flora))
    )
    .service(
        web::scope("/test")
            .service(web::resource("/").to(TestService::test))
    );

}
