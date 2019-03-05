extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

extern crate actix;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate jsonwebtoken;
extern crate lazy_static;
extern crate libreauth;
extern crate rand;
extern crate serde_json;
extern crate slug;
extern crate validator;
#[macro_use]
extern crate validator_derive;

extern crate chrono;
#[macro_use]
extern crate diesel;

mod app;
mod db;
mod error;
mod models;
mod prelude;
mod schema;
mod utils;

use actix_web::middleware::Logger;
use actix_web::server;
use std::env;

fn main() {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
        env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();

    let port = env::var("PORT").unwrap_or("8088".to_owned());
    let bind_address = format!("127.0.0.1:{}", port);

    server::new(app::create)
        .bind(&bind_address)
        .expect(&format!("Could not bind server to address {}", &bind_address))
        .start();
}
