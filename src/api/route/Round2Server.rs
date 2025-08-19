
use actix::Addr;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use crate::{db, schema};
use crate::model_round2::{Round2DataColumn, Round2DataReturnStruct};

use diesel::{QueryDsl, RunQueryDsl};
use crate::model_round1::SuccessReturnJson;
use crate::ws_actors::WsActor;

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


#[utoipa::path(
    post,
    path="/Server2/round_datas",
    request_body = Round2DataColumn,
    responses(
        (status = 200, description = "Register Round2 ScoreData", body = SuccessReturnJson),
        (status = 500, description = "Internal error")
    ),
)]
#[post("/round_datas")]
async fn postRound2Data(db:web::Data<db::Pool>,srv:web::Data<Addr<WsActor>>, item:web::Json<crate::model_round2::Round2DataColumn>)->impl Responder{
    let mut conn=db.get().unwrap();
    let new_data=crate::model_round2::Round2DataColumn{
        team_id: item.team_id,
        current_phase: item.current_phase,
        latest_down_num: item.latest_down_num,
        miss_timing: item.miss_timing,
    };
    diesel::replace_into(schema::round2_data::dsl::round2_data)
        .values(&new_data)
        .execute(&mut conn)
        .expect("Error creating Round1 data");
    HttpResponse::Ok().json(
        web::Json(SuccessReturnJson{
            status:"success".to_string()
        })
    )
}
pub fn Round2Config(cfg: &mut web::ServiceConfig){

    cfg.service(web::scope("/Server2")
                    .service(getRoundDatasR2)
                    .service(postRound2Data));
}