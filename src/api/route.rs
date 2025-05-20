mod Round1Server;
use utoipa_swagger_ui::SwaggerUi;
use actix_web::web;
use utoipa::OpenApi;
use crate::api::route;
#[derive(OpenApi)]
#[openapi(
paths(
route::Round1Server::getRoundDatasR1,
route::Round1Server::postRound1Data,
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
    ).service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-doc/openapi.json", Api_Doc::openapi()),
    );
}