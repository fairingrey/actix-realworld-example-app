use crate::db::{new_pool, DbExecutor};
use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    middleware::Logger,
    web::Data,
    web,
    App, HttpRequest,
    HttpServer,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};
use actix_cors::Cors;
use std::env;

pub mod articles;
pub mod profiles;
pub mod tags;
pub mod users;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn index(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn start() {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
        };
        let cors = match frontend_origin {
            Some(ref origin) => Cors::new()
                .allowed_origin(origin)
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
            None => Cors::new()
                .allowed_origin("*")
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
        };
        App::new()
            .register_data(Data::new(state))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(routes)
        })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .start();

    println!("You can access the server at {}", bind_address);
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(web::resource("/").to(index))
        .service(web::scope("/api")
                // User routes ↓
                .service(web::resource("users")
                    .route(web::post().to_async(users::register))
                )
                .service(web::resource("users/login")
                    .route(web::post().to_async(users::login))
                )
                .service(web::resource("user")
                    .route(web::get().to_async(users::get_current))
                    .route(web::put().to_async(users::update))
                )
                // Profile routes ↓
                .service(web::resource("profiles/{username}")
                    .route(web::get().to_async(profiles::get))
                )
                .service(web::resource("profiles/{username}/follow")
                    .route(web::post().to_async(profiles::follow))
                    .route(web::delete().to_async(profiles::unfollow))
                )
                // Article routes ↓
                .service(web::resource("articles")
                    .route(web::get().to_async(articles::list))
                    .route(web::post().to_async(articles::create))
                )
                .service(web::resource("articles/feed")
                    .route(web::get().to_async(articles::feed))
                )
                .service(web::resource("articles/{slug}")
                    .route(web::get().to_async(articles::get))
                    .route(web::put().to_async(articles::update))
                    .route(web::delete().to_async(articles::delete))
                )
                .service(web::resource("articles/{slug}/favorite")
                    .route(web::post().to_async(articles::favorite))
                    .route(web::delete().to_async(articles::unfavorite))
                )
                .service(web::resource("articles/{slug}/comments")
                    .route(web::get().to_async(articles::comments::list))
                    .route(web::post().to_async(articles::comments::add))
                )
                .service(web::resource("articles/{slug}/comments/{comment_id}")
                    .route(web::delete().to_async(articles::comments::delete))
                )
                // Tags routes ↓
                .service(web::resource("tags")
                    .route(web::get().to_async(tags::get))
                )
            );
}
