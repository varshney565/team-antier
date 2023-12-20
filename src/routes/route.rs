use actix_web::web;
use crate::controller::index::index;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/transaction")
            .route(web::get().to(index))
    );
}