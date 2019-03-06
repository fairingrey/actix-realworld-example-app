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
use crate::{
    config::Config,
    db::{
        DbExecutor,
        new_pool
    }
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
        .resource("/", |r| r.f(index))
        .scope("/api", |scope| {

            // check to enable CORS
            let scope = match frontend_origin {
                Some(ref origin) => scope.middleware(enable_cors(origin)),
                None => scope,
            };

//            // Users
//            let scope = scope
//                .resource("users", |r| r.post().with(users::sign_up))
//                .resource("users/login", |r| r.post().with(users::sign_in))
//                .resource("user", |r| {
//                    r.get().with(users::get_current);
//                    r.put().with(users::update)
//                });
//
//            // Profiles
//            let scope = scope
//                .resource("profiles/{username}", |r| r.get().with(profiles::get))
//                .resource("profiles/{username}/follow", |r| {
//                    r.post().with(profiles::follow);
//                    r.delete().with(profiles::unfollow)
//                });
//
//            // Articles
//            let scope = scope
//                .resource("articles", |r| {
//                    r.get().with(articles::list);
//                    r.post().with(articles::create)
//                }).resource("articles/feed", |r| r.get().with(articles::feed))
//                .resource("articles/{slug}", |r| {
//                    r.get().with(articles::get);
//                    r.put().with(articles::update);
//                    r.delete().with(articles::delete)
//                }).resource("articles/{slug}/favorite", |r| {
//                r.post().with(articles::favorite);
//                r.delete().with(articles::unfavorite);
//            }).resource("articles/{slug}/comments", |r| {
//                r.get().with(articles::comments::list);
//                r.post().with(articles::comments::add)
//            }).resource("articles/{slug}/comments/{id}", |r| {
//                r.delete().with(articles::comments::delete)
//            });

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
