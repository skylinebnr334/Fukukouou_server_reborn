use std::time::Instant;
use actix::Addr;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::{QueryDsl, RunQueryDsl};

use crate::actorServer_forws::WsSession_Round1Refresh;
use crate::{db, schema};
use crate::model_round1::{ErrorMsgStruct, Round1DataColumn, Round1DataReturnStruct, Round1DataReturnStruct_KOBETSU, Round1IndexRound, Round1NextRoundDT, Round1QuestionsReturnStruct, Round1QuestionsReturnStruct_KOBETSU, Round1ScoreConfigDataColumn, Round1ScoreSettingReturnStruct, SuccessReturnJson, TID};
use crate::model_round1_questions::Round1QuestionDataColumn;
use crate::ws_actors::{Round1RefreshMessage, WsActor};

pub async fn ws_route_Round1Refresh(
    req: HttpRequest,
    stream: web::Payload,
    srv:web::Data<Addr<WsActor>>,
)->Result<HttpResponse,actix_web::Error> {
    ws::start(
        WsSession_Round1Refresh{
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
path="/Server1/round_datas",
    responses(
        (status = 200, description = "Get Round1 Data", body = Round1DataReturnStruct),
        (status = 500, description = "Internal error")
    ),
)]
#[get("/round_datas")]
async fn getRoundDatasR1(db:web::Data<db::Pool>)->impl Responder{
    let mut conn=db.get().unwrap();
    let rows=schema::round1_data::table
        .load::<Round1DataColumn>(&mut conn)
        .expect("Error loading round1 data");
    let return_obj=Round1DataReturnStruct{
        result_data:rows,
    };
    HttpResponse::Ok().json(web::Json(return_obj))
}

#[utoipa::path(
    get,
    path="/Server1/questions",
    responses(
        (status = 200, description = "Get Round1 Questions", body = Round1QuestionsReturnStruct),
        (status = 500, description = "Internal error")
    ),
)]
#[get("/questions")]
async fn getRoundQuestionsR1(db:web::Data<db::Pool>)->impl Responder{
    let mut conn=db.get().unwrap();
    let rows=schema::round1_questions::table
        .load::<Round1QuestionDataColumn>(&mut conn)
        .expect("Error loading round1 questions");

    let return_obj=Round1QuestionsReturnStruct{
        result_data:rows,
    };
    HttpResponse::Ok().json(web::Json(return_obj))
}
#[utoipa::path(
    get,
    params(TID),
    path="/Server1/questions/{id}",
    responses(
        (status = 200, description = "Get Round1 Question Data", body = Round1QuestionsReturnStruct_KOBETSU),
        (status = 500, description = "Internal error")
    ),
)]
#[get("/questions/{id}")]
async fn getRoundQuestionsR1_Child(db:web::Data<db::Pool>,
                               req:web::Path<TID>)->impl Responder{
    let mut conn=db.get().unwrap();
    let Result_DT=schema::round1_questions::table
        .find(req.id)
        .first::<Round1QuestionDataColumn>(&mut conn);
    match Result_DT{
        Ok(dt)=>{

            let return_obj=Round1QuestionsReturnStruct_KOBETSU{
                result_data:dt
            };
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
    post,
    path="/Server1/questions",
    request_body = Round1QuestionDataColumn,
    responses(
        (status = 200, description = "Register Round1 ScoreData", body = SuccessReturnJson),
        (status = 500, description = "Internal error")
    ),
)]
#[post("/questions")]
async fn postRoundQuestionsR1(db:web::Data<db::Pool>,srv:web::Data<Addr<WsActor>>,item:web::Json<crate::model_round1_questions::Round1QuestionDataColumn>)->impl Responder{
    let mut conn=db.get().unwrap();
    let new_round_data=crate::model_round1_questions::Round1QuestionDataColumn{
        stageno: item.stageno,
        answer: item.answer.clone(),
        question: item.question.clone(),
        comment: item.comment.clone(),
    };
    diesel::replace_into(schema::round1_questions::dsl::round1_questions)
        .values(&new_round_data)
        .execute(&mut conn)
        .expect("Error creating Round1 Questions");
    HttpResponse::Ok().json(
        web::Json(SuccessReturnJson{
            status:"success".to_string()
        })
    )
}




#[utoipa::path(
    get,
    params(TID),
    path="/Server1/round_datas/{id}",
    responses(
        (status = 200, description = "Get Round1 Data", body = Round1DataColumn),
        (status = 500, description = "Internal error")
    ),
)]
#[get("/round_datas/{id}")]
async fn getRoundDatasR1_Child(db:web::Data<db::Pool>,
req:web::Path<TID>)->impl Responder{
    let mut conn=db.get().unwrap();
    let Result_DT=schema::round1_data::table
        .find(req.id)
        .first::<Round1DataColumn>(&mut conn);
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
    post,
    path="/Server1/round_datas",
    request_body = Round1DataColumn,
    responses(
        (status = 200, description = "Register Round1 ScoreData", body = SuccessReturnJson),
        (status = 500, description = "Internal error")
    ),
)]
#[post("/round_datas")]
async fn postRound1Data(db:web::Data<db::Pool>,srv:web::Data<Addr<WsActor>>,item:web::Json<crate::model_round1::Round1DataColumn>)->impl Responder{
    let mut conn=db.get().unwrap();
    let new_round_data=crate::model_round1::Round1DataColumn{
        id:item.id,
        team1:item.team1,
        team2:item.team2,
        team3:item.team3,
        team4:item.team4,
        team5:item.team5,
        team6:item.team6,
    };
    diesel::replace_into(schema::round1_data::dsl::round1_data)
        .values(&new_round_data)
        .execute(&mut conn)
        .expect("Error creating Round1 data");
    srv.get_ref().do_send(Round1RefreshMessage {msg:"refresh".parse().unwrap() });
    HttpResponse::Ok().json(
        web::Json(SuccessReturnJson{
            status:"success".to_string()
        })
    )
}

#[utoipa::path(
    get,
    path="/Server1/score_setting",
    responses(
        (status = 200, description = "Get Score Setting Data", body = Round1ScoreSettingReturnStruct),
        (status = 500, description = "Internal error")
    ),
)]
#[get("/score_setting")]
async fn get_score_settingRound1(db:web::Data<db::Pool>)->impl Responder{

    let mut conn=db.get().unwrap();
    let rows=schema::round1_tokutendt::table
        .load::<Round1ScoreConfigDataColumn>(&mut conn)
        .expect("Error loading round1 Score");
    let return_obj=Round1ScoreSettingReturnStruct{
        result_data:rows,
    };
    HttpResponse::Ok().json(web::Json(return_obj))
}


#[post("/score_setting")]
async fn postScore_settingRound1(db:web::Data<db::Pool>,item:web::Json<crate::model_round1::Round1ScoreConfigDataColumn>)->impl Responder{
    let mut conn=db.get().unwrap();
    let new_scorecf_data=crate::model_round1::Round1ScoreConfigDataColumn{
        id:item.id,
        ask_throw:item.ask_throw,
        correct:item.correct,
        miss:item.miss
    };
    diesel::replace_into(schema::round1_tokutendt::dsl::round1_tokutendt)
        .values(&new_scorecf_data)
        .execute(&mut conn)
        .expect("Error creating Round1 Score Config");

    HttpResponse::Ok().json(
        web::Json(SuccessReturnJson{
            status:"success".to_string()
        })
    )
}

#[utoipa::path(
    get,
    path="/Server1/next_round",
    responses(
        (status = 200, description = "Get Next Stage Data",body = Round1NextRoundDT),
        (status = 500, description = "Internal error")
    ),
)]
#[get("/next_round")]
async fn getNextRound1(db:web::Data<db::Pool>)->impl Responder{

    let mut conn=db.get().unwrap();
    let rows=schema::round1_info::table
        .load::<Round1IndexRound>(&mut conn)
        .expect("Error loading round1 stage");
    for n in rows{

        return HttpResponse::Ok().json(web::Json(Round1NextRoundDT{
            current_stage:n.current_stage
        }))
    }HttpResponse::Ok().json(web::Json(Round1NextRoundDT{
        current_stage:0
    }))
}



#[utoipa::path(
    post,
    path="/Server1/next_round",
    request_body = crate::model_round1::Round1NextRoundDT,
    responses(
        (status = 200, description = "Set Round1 Next Stage", body = SuccessReturnJson),
        (status = 500, description = "Internal error")
    ),
)]
#[post("/next_round")]
async fn postNextRound1(db:web::Data<db::Pool>,item:web::Json<crate::model_round1::Round1NextRoundDT>)->impl Responder{
    let mut conn=db.get().unwrap();
    let new_RD=crate::model_round1::Round1IndexRound{
        id:0,
        current_stage:item.current_stage
    };
    diesel::replace_into(schema::round1_info::dsl::round1_info)
        .values(&new_RD)
        .execute(&mut conn)
        .expect("Error creating Round1 Stage Config");
    HttpResponse::Ok().json(
        web::Json(SuccessReturnJson{
            status:"success".to_string()
        })
    )
}


pub fn Round1config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/Server1")
        .service(getRoundDatasR1)
        .service(postRound1Data)
        .service(getRoundDatasR1_Child)
        .service(get_score_settingRound1)
        .service(postScore_settingRound1)
        .service(getNextRound1)
        .service(postNextRound1)
        .service(getRoundQuestionsR1)
        .service(getRoundQuestionsR1_Child)
        .service(postRoundQuestionsR1)
        .service(web::resource("/round1_ws").to(ws_route_Round1Refresh))
    );

}