mod Round1Server;
mod Round2Server;

use utoipa_swagger_ui::SwaggerUi;
use actix_web::web;
use utoipa::OpenApi;
use crate::api::route;
#[derive(OpenApi)]
#[openapi(
paths(
route::Round1Server::getRoundDatasR1, 
    route::Round1Server::getRoundDatasR1_Child,
route::Round1Server::postRound1Data,
route::Round1Server::get_score_settingRound1,
route::Round1Server::getNextRound1,
route::Round1Server::postNextRound1,
route::Round1Server::getRoundQuestionsR1,
    route::Round1Server::getRoundQuestionsR1_Child,
route::Round1Server::postRoundQuestionsR1,
    route::Round2Server::getRoundDatasR2,
route::Round2Server::postRound2Data,
route::Round2Server::getNextRound2,
route::Round2Server::postNextRound2,
    route::Round2Server::get_round2data_by_id,
route::Round2Server::postRound2Data_PLUS,
route::Round1Server::getRound1UsedQuestions,
route::Round1Server::postRound1UsedQuestion,
route::Round1Server::get_PlayStart,
route::Round1Server::get_StopVideo,
    route::Round1Server::post_PlayStart,
    route::Round1Server::post_StopVideo
),
components(
    schemas(
        crate::model_round1::Round1DataColumn,
        crate::model_round1::Round1DataReturnStruct
    )
)
)]
struct Api_Doc;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(
        Round1Server::Round1config
    ).configure(Round2Server::Round2Config)
        .service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-doc/openapi.json", Api_Doc::openapi()),
    );
}