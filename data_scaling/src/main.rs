use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::io;

mod routes;
mod services;

mod models;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
