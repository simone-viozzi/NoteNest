pub mod checklist;
pub mod notes;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    log::debug!("Initializing notes routes...");
    notes::init_routes(cfg);
    log::debug!("Initializing checklist routes...");
    checklist::init_routes(cfg);
}
