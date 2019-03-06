use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    http::{header, StatusCode},
    middleware::{
        cors::Cors,
        Logger
    },
    App,
    HttpRequest,
};
use crate::db::{
    DbExecutor,
    new_pool
};
use std::env;

//mod articles;
//mod error;
//mod profiles;
//mod res;
//mod tags;
//mod users;

const NUM_DB_THREADS: usize = 4;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "Hello world!"
}

pub fn create() -> App<AppState> {

    let jwt_secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");

    let database_address = SyncArbiter::start(NUM_DB_THREADS, move || DbExecutor(database_pool.clone()));

    let state = AppState {
        db: database_address.clone(),
    };

    App::with_state(state)
        .middleware(Logger::default())
        .resource("/", |r| r.f(index))
        .scope("/api", |scope| {

            // check to enable CORS
            let scope = match frontend_origin {
                Some(ref origin) => scope.middleware(enable_cors(origin)),
                None => scope,
            };

            scope
        })
}

fn enable_cors(origin: &str) -> Cors {
    Cors::build()
        .allowed_origin(origin)
        .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(3600)
        .finish()
}
