#![allow(unused_must_use)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod app;
mod db;
mod error;
mod models;
mod prelude;
mod schema;
mod utils;

use std::env;

fn main() {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }
    env_logger::init();
    app::start();
}
