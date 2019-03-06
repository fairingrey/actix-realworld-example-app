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
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate lazy_static;
extern crate libreauth;
extern crate rand;
extern crate regex;
extern crate serde_json;
extern crate slug;
extern crate validator;
#[macro_use]
extern crate validator_derive;

extern crate chrono;
#[macro_use]
extern crate diesel;

mod app;
mod config;
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
    }
    env_logger::init();

    let sys = actix::System::new("conduit");

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    println!("You can access the server at {}", bind_address);

    server::new(app::create)
        .bind(&bind_address)
        .expect(&format!("Could not bind server to address {}", &bind_address))
        .start();

    let _ = sys.run();
}
