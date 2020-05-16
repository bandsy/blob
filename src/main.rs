use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;

mod image;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //let host = env::var("host").expect("host not set");
    //let port = env::var("port").expect("port not set");
    let host = String::from("127.0.0.1");
    let port = String::from("8000");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(image::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
