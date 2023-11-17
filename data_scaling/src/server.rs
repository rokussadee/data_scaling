use actix_web::{App, HttpServer};
use crate::routes;
use crate::database::add_pool;

pub async fn server() -> std::io::Result<()> {
    
    let mut server = HttpServer::new(move || {
        App::new()
            .configure(add_pool)
            .configure(routes::configure_routes)
    });
    

    server
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
