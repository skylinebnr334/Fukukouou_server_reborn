# [macro_use]
extern crate log;
extern crate env_logger as logger;
use log::Level;
use std::env;


#[macro_use]
extern crate diesel;

mod schema;
mod model_round1;
mod db;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn rootpage()->impl Responder{
    HttpResponse::Ok().body("root page")
}

#[actix_web::main]
async fn main()->std::io::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "info");
    }
    logger::init();
    info!("Fukukouou Server v{}", env!("CARGO_PKG_VERSION"));

    HttpServer::new(|| {
        App::new()
            .service(rootpage)
    })
        .bind(("127.0.0.1", 8080))?
    .run()
        .await
}