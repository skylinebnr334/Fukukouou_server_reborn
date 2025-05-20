mod Round1Server;
use utoipa_swagger_ui::SwaggerUi;
use actix_web::web;
use utoipa::OpenApi;

#[derive(OpenApi)]
struct Api_Doc;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(
        Round1Server::Round1config
    ).service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-doc/opanapi.json", Api_Doc::openapi()),
    );
}