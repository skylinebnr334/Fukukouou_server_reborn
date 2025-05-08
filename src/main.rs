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

use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use diesel::RunQueryDsl;
use crate::model_round1::{Round1DataColumn, Round1DataReturnStruct, SuccessReturnJson};

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
        result_data:rows,
    };
    HttpResponse::Ok().json(web::Json(return_obj))
}

#[post("/Server1/set_round_data")]
async fn postRound1Data(db:web::Data<db::Pool>,item:web::Json<model_round1::Round1DataColumn>)->impl Responder{
    let mut conn=db.get().unwrap();
    let new_round_data=model_round1::Round1DataColumn{
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
    HttpResponse::Ok().json(
        web::Json(SuccessReturnJson{
            status:"success".to_string()
        })
    )
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
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(rootpage)
            .service(getRoundDatasR1)
            .service(postRound1Data)
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

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    #[actix_web::test]
    async fn test_Round1ScoreSettings() {
        let pool = db::establish_connection_for_test();
        pool.get().unwrap().run_pending_migrations(MIGRATIONS);

        let app = test::init_service(App::new().app_data
        (Data::new(pool.clone()))
            .service(rootpage)
            .service(getRoundDatasR1)
            .service(postRound1Data)
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
        let Round1DataPostReq=test::TestRequest::post().uri("/Server1/set_round_data").set_json(web::Json(
            Round1SetData.clone()
        )).to_request();
        let Round1DataPostresp = test::call_service(&app, Round1DataPostReq).await;

        let Round1DataReq=test::TestRequest::get().uri("/Server1/get_round_datas").to_request();
        let Round1Dataresp = test::call_service(&app, Round1DataReq).await;
        let Round1DataResp_Soutei=Round1DataReturnStruct{
            result_data:vec![Round1SetData]
        };
        compare_JS(Round1Dataresp,
        Round1DataResp_Soutei).await;


    }
    #[actix_web::test]
    async fn test_Round1Data() {
        let pool = db::establish_connection_for_test();
        pool.get().unwrap().run_pending_migrations(MIGRATIONS);

        let app = test::init_service(App::new().app_data
        (Data::new(pool.clone()))
            .service(rootpage)
            .service(getRoundDatasR1)
            .service(postRound1Data)
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
        let Round1DataPostReq=test::TestRequest::post().uri("/Server1/set_round_data").set_json(web::Json(
            Round1SetData.clone()
        )).to_request();
        let Round1DataPostresp = test::call_service(&app, Round1DataPostReq).await;

        let Round1DataReq=test::TestRequest::get().uri("/Server1/get_round_datas").to_request();
        let Round1Dataresp = test::call_service(&app, Round1DataReq).await;
        let Round1DataResp_Soutei=Round1DataReturnStruct{
            result_data:vec![Round1SetData]
        };
        compare_JS(Round1Dataresp,
                   Round1DataResp_Soutei).await;


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