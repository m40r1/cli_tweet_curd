mod handlers;
mod models;
mod schema;

use actix_web::{middleware, web, App, HttpServer};
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// a postrgres connection String
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // the connection str is in .env
    dotenv::dotenv().ok();
    // gets db login
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // Creates new postgres connection
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // for async i guess
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool");
    // better logs
    // if i want to max perfomance
    // this goes out
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // the http server
    HttpServer::new(move || {
        App::new()
            // access to pool
            .app_data(web::Data::new(pool.clone()))
            // gets another logger
            .wrap(middleware::Logger::default())
            // this is all the services
            // in handler.rs
            .route("/", web::get().to(|| async { "Actix REST API\n" }))
            .service(handlers::index)
            .service(handlers::create)
            .service(handlers::show)
            .service(handlers::update)
            .service(handlers::delete)
    })
    // where its serving
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
