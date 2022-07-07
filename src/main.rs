mod api;

use actix_web::{middleware::Logger, web, App, HttpServer};
use api::hello::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api").service(hello))
    })
    .bind(("127.0.0.1", 1337))?
    .run()
    .await
}
