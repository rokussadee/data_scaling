use actix_web::{App, HttpServer, web::Data};
use dotenv::dotenv;
use crate::routes;
use crate::database::add_pool;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use std::env;
use actix::SyncArbiter;
use crate::db_utils::{get_pool, AppState, DbActor};

pub async fn server() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: db_addr.clone() }))
            .configure(add_pool)
            .configure(routes::configure_routes)
    });
    

    server
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
