use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::io;
use actix::SyncArbiter;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

use crate::server::server;

//mod db_models;
//mod actors;
mod insertables;
mod db_utils;
mod messages;
mod schema;
mod server;
mod routes;
mod services;
mod database;
mod config;
mod models;
mod dto;

use db_utils::{get_pool, AppState, DbActor};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    server().await
}
