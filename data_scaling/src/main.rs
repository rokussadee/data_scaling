use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::io;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;


use crate::server::server;

mod server;
mod routes;
mod services;
mod database;
mod config;

mod models;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    server().await
}
