use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    http::{header, Method, StatusCode},
    middleware::{
        cors::Cors,
        Logger
    },
    App,
    HttpRequest,
};
use crate::{
    config::Config,
    db::{
        DbExecutor,
        new_pool
    }
};
use std::env;

mod articles;
mod profiles;
mod tags;
mod users;

const NUM_DB_THREADS: usize = 4;

pub struct AppState {
    pub config: Config,
    pub db: Addr<DbExecutor>,
}

fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "Hello world!"
}

pub fn create() -> App<AppState> {

    let jwt_secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();
    let config = Config {
        jwt_secret_key: jwt_secret_key.clone(),
        frontend_origin: frontend_origin.clone(),
    };

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");

    let database_address = SyncArbiter::start(NUM_DB_THREADS, move || DbExecutor(database_pool.clone()));

    let state = AppState {
        config,
        db: database_address.clone(),
    };

    App::with_state(state)
        .middleware(Logger::default())
        .configure(|app| {
            // check whether to enable CORS
            match frontend_origin {
                Some(ref origin) => {
                    Cors::for_app(app)
                        .allowed_origin(origin)
                        .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                        .max_age(3600)
                        .register()
                }
                None => app
            }
        })
        .resource("/", |r| r.f(index))
        .scope("/api", |scope| {

            // Users
            let scope = scope
                .resource("users", |r| {
                    r.method(Method::POST).with_async(users::sign_up)
                });

            scope
        })
}
