# [macro_use]
extern crate log;
extern crate env_logger as logger;
use log::Level;
use std::env;
use std::time::Instant;
use actix::{Actor, Addr};

#[macro_use]
extern crate diesel;

mod schema;
mod model_round1;
mod model_round1_questions;
mod db;
mod ws_actors;
mod actorServer_forws;
mod api;
mod model_round2;

use actix_web::{get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use actix_web_actors::ws;
use diesel::RunQueryDsl;
use crate::actorServer_forws::{WsSession_Round1Refresh, WsSession_Round2Refresh};
use crate::api::route::config;
use crate::model_round1::{Round1DataColumn, Round1DataReturnStruct, Round1IndexRound, Round1ScoreConfigDataColumn, Round1ScoreSettingReturnStruct, SuccessReturnJson};
use crate::ws_actors::{Round1RefreshMessage, WsActor};




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





#[get("/")]
async fn rootpage(db:web::Data<db::Pool>)->impl Responder{
    HttpResponse::Ok().body("root page")
}
#[actix_web::main]
async fn main()->std::io::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "info");
    }
    logger::init();
    info!("Fukukouou Server v{}", env!("CARGO_PKG_VERSION"));
    let pool=db::establish_connection();

    let ws_server = WsActor::new().start();

/*
    HttpServer::new(move ||
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(ws_server.clone()))
            .service(rootpage)
            .service(getRoundDatasR1)
            .service(postRound1Data)
            .service(get_score_settingRound1)
            .service(postScore_settingRound1)
            .service(getNextRound1)
            .service(postNextRound1)
            .service(web::resource("/Server1/round1_ws").to(ws_route_Round1Refresh))
            .service(web::resource("/Server2/round2_ws").to(ws_route_Round2Refresh))

    )*/
    HttpServer::new(move ||
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(ws_server.clone()))
            .service(rootpage)
            .configure(config)
            .service(web::resource("/Server2/round2_ws").to(ws_route_Round2Refresh))

    )
        .bind(("127.0.0.1", 8080))?
    .run()
        .await
}



//tests

#[cfg(test)]
mod unit_dbtest{

    use super::*;
    use actix_web::dev::{ServiceResponse, WebService};
    use actix_web::{http, test};
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;
    use serde_json::{Value, json};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use crate::model_round1::{Round1NextRoundDT, Round1QuestionsReturnStruct, Round1QuestionsReturnStruct_KOBETSU};
    use crate::model_round1_questions::Round1QuestionDataColumn;
    use crate::model_round2::{Round2DataColumn, Round2DataReturnStruct};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    #[actix_web::test]
    async fn test_Round1ScoreSettings() {
        let pool = db::establish_connection_for_test();
        pool.get().unwrap().run_pending_migrations(MIGRATIONS);

        let app = test::init_service(App::new().app_data
        (Data::new(pool.clone()))
            .configure(config)
        ).await;

        let Round1SetScore=Round1ScoreConfigDataColumn{
          id:0,
            miss:-1,
            correct:1,
            ask_throw:0,
        };
        let Round1ScorePostReq=test::TestRequest::post().uri("/Server1/score_setting").set_json(web::Json(
            Round1SetScore.clone()
        )).to_request();
        let Round1ScorePostresp = test::call_service(&app, Round1ScorePostReq).await;

        let Round1ScoreReq=test::TestRequest::get().uri("/Server1/score_setting").to_request();
        let Round1Scoreresp = test::call_service(&app, Round1ScoreReq).await;
        let Round1ScoreResp_Soutei=Round1ScoreSettingReturnStruct{
            result_data:vec![Round1SetScore]
        };
        compare_JS(Round1Scoreresp,
                   Round1ScoreResp_Soutei).await;


    }


    #[actix_web::test]
    async fn test_Round1StageInfo() {
        let pool = db::establish_connection_for_test();
        pool.get().unwrap().run_pending_migrations(MIGRATIONS);

        let app = test::init_service(App::new().app_data
        (Data::new(pool.clone()))
            .configure(config)
        ).await;

        let Round1StageeReq_1=test::TestRequest::get().uri("/Server1/next_round").to_request();
        let Round1Stageresp_1 = test::call_service(&app, Round1StageeReq_1).await;
        let Round1ScoreResp_Soutei_1=Round1NextRoundDT{
                current_stage:0
        };
        //assert_eq!(String::from_utf8(to_bytes(Round1Stageresp_1.into_body()).await.unwrap().to_vec()).unwrap(),Round1ScoreResp_Soutei_1.to_string());
        compare_JS(Round1Stageresp_1,Round1ScoreResp_Soutei_1);

        let Round1SetStage=Round1NextRoundDT{
            current_stage:6
        };
        let Round1StagePostReq=test::TestRequest::post().uri("/Server1/next_round").set_json(web::Json(
            Round1SetStage.clone()
        )).to_request();
        let Round1StagePostresp = test::call_service(&app, Round1StagePostReq).await;
        let Round1StageeReq_2=test::TestRequest::get().uri("/Server1/next_round").to_request();
        let Round1Stageresp_2 = test::call_service(&app, Round1StageeReq_2).await;
        let Round1ScoreResp_Soutei_2=Round1NextRoundDT{
            current_stage:6
        };
        //assert_eq!(String::from_utf8(to_bytes(Round1Stageresp_2.into_body()).await.unwrap().to_vec()).unwrap(),Round1ScoreResp_Soutei_2.to_string());
        compare_JS(Round1Stageresp_2,Round1ScoreResp_Soutei_2);

        let Round1SetStage_3=Round1NextRoundDT{
            current_stage:-1
        };
        let Round1StagePostReq_3=test::TestRequest::post().uri("/Server1/next_round").set_json(web::Json(
            Round1SetStage_3.clone()
        )).to_request();
        let Round1StagePostresp_3 = test::call_service(&app, Round1StagePostReq_3).await;
        let Round1StageeReq_3=test::TestRequest::get().uri("/Server1/next_round").to_request();
        let Round1Stageresp_3 = test::call_service(&app, Round1StageeReq_3).await;
        let Round1ScoreResp_Soutei_3=Round1SetStage_3;
        //assert_eq!(String::from_utf8(to_bytes(Round1Stageresp_3.into_body()).await.unwrap().to_vec()).unwrap(),Round1ScoreResp_Soutei_3.to_string());
        compare_JS(Round1Stageresp_3,Round1ScoreResp_Soutei_3);


    }

    #[actix_web::test]
    async fn test_Round1Data() {
        let pool = db::establish_connection_for_test();
        pool.get().unwrap().run_pending_migrations(MIGRATIONS);

        let ws_server = WsActor::new().start();
        let app = test::init_service(App::new().app_data
        (Data::new(pool.clone()))
            .app_data(Data::new(ws_server.clone()))
            .service(rootpage)
            .configure(config)
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp.response().body());

        assert_eq!(resp.status(), StatusCode::OK);

        let Round1SetData=Round1DataColumn{
            id:0,
            team1:1,
            team2:2,
            team3:0,
            team4:1,
            team5:2,
            team6:1,
        };
        let Round1DataPostReq=test::TestRequest::post().uri("/Server1/round_datas").set_json(web::Json(
            Round1SetData.clone()
        )).to_request();
        let Round1DataPostresp = test::call_service(&app, Round1DataPostReq).await;

        let Round1DataReq=test::TestRequest::get().uri("/Server1/round_datas").to_request();
        let Round1Dataresp = test::call_service(&app, Round1DataReq).await;
        let Round1DataResp_Soutei=Round1DataReturnStruct{
            result_data:vec![Round1SetData]
        };
        compare_JS(Round1Dataresp,
                   Round1DataResp_Soutei).await;


    }


    #[actix_web::test]
    async fn test_Round1QuestionData() {
        let pool = db::establish_connection_for_test();
        pool.get().unwrap().run_pending_migrations(MIGRATIONS);

        let ws_server = WsActor::new().start();
        let app = test::init_service(App::new().app_data
        (Data::new(pool.clone()))
            .app_data(Data::new(ws_server.clone()))
            .service(rootpage)
            .configure(config)
        ).await;
        let Round1QSetData=Round1QuestionDataColumn{
            stageno:0,
            question:"Question1".to_string(),
            answer:"Answer1".to_string(),
            comment:"Comment1".to_string(),
        };
        let Round1QDataPostReq=test::TestRequest::post().uri("/Server1/questions").set_json(web::Json(
            Round1QSetData.clone()
        )).to_request();
        let Round1QDataPostresp = test::call_service(&app, Round1QDataPostReq).await;

        let Round1QDataReq1=test::TestRequest::get().uri("/Server1/questions").to_request();
        let Round1QDataresp1 = test::call_service(&app, Round1QDataReq1).await;
        let Round1QDataResp_Soutei=Round1QuestionsReturnStruct{
            result_data:vec![Round1QSetData.clone()]
        };
        let Round1QDataResp_Soutei2=Round1QuestionsReturnStruct_KOBETSU{
            result_data:Round1QSetData
        };
        compare_JS(Round1QDataresp1,
                   Round1QDataResp_Soutei).await;
        let Round1QDataReq2=test::TestRequest::get().uri("/Server1/questions/0").to_request();
        let Round1QDataresp2 = test::call_service(&app, Round1QDataReq2).await;

        compare_JS(Round1QDataresp2,
                   Round1QDataResp_Soutei2).await;


    }

    #[actix_web::test]
    async fn test_Round2Data() {
        let pool = db::establish_connection_for_test();
        pool.get().unwrap().run_pending_migrations(MIGRATIONS);

        let ws_server = WsActor::new().start();
        let app = test::init_service(App::new().app_data
        (Data::new(pool.clone()))
            .app_data(Data::new(ws_server.clone()))
            .service(rootpage)
            .configure(config)
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp.response().body());

        assert_eq!(resp.status(), StatusCode::OK);

        let Round2SetData=Round2DataColumn{
            team_id:0,
            current_phase:2,
            latest_down_num:1,
            miss_timing:0,
        };
        let Round2DataPostReq=test::TestRequest::post().uri("/Server2/round_datas").set_json(web::Json(
            Round2SetData.clone()
        )).to_request();
        let Round2DataPostresp= test::call_service(&app, Round2DataPostReq).await;


        let Round2SetData_2=Round2DataColumn{
            team_id:1,
            current_phase:3,
            latest_down_num:2,
            miss_timing:2,
        };
        let Round2DataPostReq_2=test::TestRequest::post().uri("/Server2/round_datas").set_json(web::Json(
            Round2SetData_2.clone()
        )).to_request();
        let Round2DataPostresp_2= test::call_service(&app, Round2DataPostReq_2).await;
        let Round2DataReq=test::TestRequest::get().uri("/Server2/round_datas").to_request();
        let Round2Dataresp=test::call_service(&app, Round2DataReq).await;
        let Round2DataResp_Soutei=Round2DataReturnStruct{
            result_data:vec![Round2SetData,Round2SetData_2.clone()]
        };
        compare_JS(Round2Dataresp,
                   Round2DataResp_Soutei).await;
        let Round2_KobetuDataReq=test::TestRequest::get().uri("/Server2/round_datas/1").to_request();
        let Round2_KobetuDataresp=test::call_service(&app, Round2_KobetuDataReq).await;
        let Round2_KobetuDataResp_Soutei=Round2SetData_2;
        compare_JS(Round2_KobetuDataresp,
                   Round2_KobetuDataResp_Soutei).await;

    }

    async fn compare_JS(res:ServiceResponse,obj:impl serde::Serialize){
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>
            (&to_bytes(res.into_body()).await.unwrap())
            .unwrap(),
            serde_json::to_value(obj).unwrap()
        )
    }
}