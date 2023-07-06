use actix_web::{web, HttpResponse, Responder};
use rust_wheel::model::response::api_response::ApiResponse;

use crate::service::app::app_service::get_app_list;

#[derive(serde::Deserialize)]
pub struct AppParams {
    tag: String,
}

pub async fn get_apps_by_tag(params: web::Query<AppParams>) -> impl Responder {
    let tags_list = get_app_list(&params.tag);
    let res = ApiResponse {
        result: tags_list,
        ..Default::default()
    };
    HttpResponse::Ok().json(res)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/alt/app").route("/list", web::get().to(get_apps_by_tag)));
}
