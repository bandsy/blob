use crate::image::Image;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/v1/find/{id}")]
async fn find(path: web::Path<(i32,)>) -> impl Responder {
    let id = path.0;

    HttpResponse::Ok().json({
        Image {
            id: id,
            file_name: String::from("yeet"),
        }
    })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
}
