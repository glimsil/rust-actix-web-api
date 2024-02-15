mod domain;
mod infrastructure;

use infrastructure::api::health::{
    get_health,
};

use actix_web::{HttpServer, App, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(get_health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
