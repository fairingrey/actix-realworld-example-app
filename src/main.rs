#[macro_use]
extern crate failure;

#[macro_use]
extern crate diesel;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate validator_derive;

mod app;
mod db;
mod error;
mod models;
mod prelude;
mod schema;
mod utils;

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
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .start();

    let _ = sys.run();
}
