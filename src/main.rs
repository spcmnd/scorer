#[macro_use]
extern crate diesel;

mod api;
mod models;
mod schema;
mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};
use api::{
    hello::hello,
    player::{get_players, post_player},
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};

fn get_database_pool(database_url: String) -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pool = get_database_pool(database_url);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(hello)
                    .service(post_player)
                    .service(get_players),
            )
    })
    .bind(("127.0.0.1", 1337))?
    .run()
    .await
}
