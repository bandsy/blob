#[macro_use]
extern crate log;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod image;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("host").expect("host not set");
    let port = env::var("port").expect("port not set");

    info!("starting server");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(image::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
