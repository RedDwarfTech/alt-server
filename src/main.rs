use actix_web::{ web, App, HttpServer};
use controller::app::alt_app_controller::{hello, echo, manual_hello};
use sched::scrapy::check_tpl_task;

pub mod sched;
pub mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let task: tokio::task::JoinHandle<()> = tokio::spawn(check_tpl_task());
    match task.await {
        Ok(_) => println!("success!"),
        Err(_) => println!("schedule task failed!"),
    };

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            //.route("/path", web::get().to(get_app_by_tag))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}