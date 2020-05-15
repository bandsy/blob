use actix_web::{middleware, App, HttpServer};

mod image;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(image::init_routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
