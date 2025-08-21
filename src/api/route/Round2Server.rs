use std::time::Instant;
use actix::Addr;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use crate::{db, schema};
use crate::model_round2::{Round2DataColumn, Round2DataReturnStruct, Round2DataReturnStruct_KOBETSU, Round2IndexRound, Round2NextRoundDT};

use diesel::{QueryDsl, RunQueryDsl};
use crate::actorServer_forws::{WsSession_Round2Refresh};
use crate::model_round1::{ErrorMsgStruct, SuccessReturnJson, TID};
use crate::ws_actors::{Round2RefreshMessage, WsActor};

pub async fn ws_route_Round2Refresh(
    req: HttpRequest,
    stream: web::Payload,
    srv:web::Data<Addr<WsActor>>,
)->Result<HttpResponse,actix_web::Error> {
    ws::start(
        WsSession_Round2Refresh{
            id:0,
            hb:Instant::now(),
            addr:srv.get_ref().clone(),
        },
        &req,
        stream
    )
}
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
    srv.get_ref().do_send(Round2RefreshMessage {msg:"refresh".parse().unwrap() });
    HttpResponse::Ok().json(
        web::Json(SuccessReturnJson{
            status:"success".to_string()
        })
    )
}
#[utoipa::path(
    get,
    params(TID),
    path="/Server2/round_datas/{id}",
    responses(
        (status = 200, description = "Get Round2 Data", body = Round2DataColumn),
        (status = 500, description = "Internal error")
    ),
)]
#[get("/round_datas/{id}")]
async fn get_round2data_by_id(db:web::Data<db::Pool>,
                              req:web::Path<TID>) ->impl Responder{
    let mut conn=db.get().unwrap();
    let Result_DT=schema::round2_data::table
        .find(req.id)
        .first::<Round2DataColumn>(&mut conn);
    match Result_DT{
        Ok(dt)=>{

            let return_obj=dt;
            HttpResponse::Ok().json(web::Json(return_obj))
        }
        Err(err)=>{
            HttpResponse::InternalServerError().json(web::Json(ErrorMsgStruct{
                error_shortmsg:"DB Error".parse().unwrap(),
                error_msg:err.to_string()
            }))
        }
    }
}
#[utoipa::path(
    get,
    path="/Server2/next_round",
    responses(
        (status = 200, description = "Get Next Stage Data",body = Round2NextRoundDT),
        (status = 500, description = "Internal error")
    ),
)]
#[get("/next_round")]
async fn getNextRound2(db:web::Data<db::Pool>)->impl Responder{

    let mut conn=db.get().unwrap();
    let rows=schema::round2_info::table
        .load::<Round2IndexRound>(&mut conn)
        .expect("Error loading round2 stage");
    for n in rows{

        return HttpResponse::Ok().json(web::Json(Round2NextRoundDT{
            current_num:n.current_num
        }))
    }HttpResponse::Ok().json(web::Json(Round2NextRoundDT{
        current_num:0
    }))
}
#[utoipa::path(
    post,
    path="/Server2/next_round",
    request_body = crate::model_round2::Round2NextRoundDT,
    responses(
        (status = 200, description = "Set Round2 Next Stage", body = SuccessReturnJson),
        (status = 500, description = "Internal error")
    ),
)]
#[post("/next_round")]
async fn postNextRound2(db:web::Data<db::Pool>,req:web::Json<Round2NextRoundDT>)->impl Responder{
    let mut conn=db.get().unwrap();
    let new_data=Round2IndexRound{
        id:0,
        current_num:req.current_num,
    };
    diesel::replace_into(schema::round2_info::dsl::round2_info)
        .values(&new_data)
        .execute(&mut conn)
        .expect("Error creating Round2 data");
    HttpResponse::Ok().json(
        web::Json(SuccessReturnJson{
            status:"success".to_string()
        })
    )
}
pub fn Round2Config(cfg: &mut web::ServiceConfig){

    cfg.service(web::scope("/Server2")
                    .service(getRoundDatasR2)
                    .service(postRound2Data)
    .service(get_round2data_by_id)
        .service(getNextRound2)
        .service(postNextRound2)
                    .service(web::resource("/round2_ws").to(ws_route_Round2Refresh)));
}