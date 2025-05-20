mod Round1Server;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(
        Round1Server::Round1config
    );
}