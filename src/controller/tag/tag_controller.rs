use actix_web::{ HttpResponse, Responder, web};

pub async fn get_tags() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tag")
            .route("/list", web::get().to(get_tags))
    );
}
