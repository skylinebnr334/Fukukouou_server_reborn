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
use actix_web::web::Data;
use diesel::RunQueryDsl;
use crate::model_round1::{Round1DataColumn, Round1DataReturnStruct};

#[get("/")]
async fn rootpage(db:web::Data<db::Pool>)->impl Responder{
    HttpResponse::Ok().body("root page")
}
#[get("/Server1/get_round_datas")]
async fn getRoundDatasR1(db:web::Data<db::Pool>)->impl Responder{
    let mut conn=db.get().unwrap();
    let rows=schema::round1_data::table
        .load::<Round1DataColumn>(&mut conn)
        .expect("Error loading round1 data");
    let return_obj=Round1DataReturnStruct{
        result_data:vec![],
    };
    HttpResponse::Ok().json(web::Json(return_obj))
}

#[actix_web::main]
async fn main()->std::io::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "info");
    }
    logger::init();
    info!("Fukukouou Server v{}", env!("CARGO_PKG_VERSION"));
    let pool=db::establish_connection();

    HttpServer::new(move ||
        App::new().app_data(Data::new(pool.clone()))
            .service(rootpage)
            .service(getRoundDatasR1)
    )
        .bind(("127.0.0.1", 8080))?
    .run()
        .await
}