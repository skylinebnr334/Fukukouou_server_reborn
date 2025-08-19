
use actix::Addr;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use crate::{db, schema};
use crate::model_round2::{Round2DataColumn, Round2DataReturnStruct};

use diesel::{QueryDsl, RunQueryDsl};

#[utoipa::path(
    get,
    path="/Server2/round_datas",
    responses(
        (status = 200, description = "Get Round2 Data", body = Round2DataReturnStruct),
        (status = 500, description = "Internal error")
    ),
)]
#[get("/round_datas")]
async fn getRoundDatasR2(db:web::Data<db::Pool>)->impl Responder{
    let mut conn=db.get().unwrap();
    let rows=schema::round2_data::table
        .load::<Round2DataColumn>(&mut conn)
        .expect("Error loading round1 data");
    let return_obj=Round2DataReturnStruct{
        result_data:rows,
    };
    HttpResponse::Ok().json(web::Json(return_obj))
}
pub fn Round2Config(cfg: &mut web::ServiceConfig){

    cfg.service(web::scope("/Server2")
                    .service(getRoundDatasR2));
}