pub mod data;

pub use data::DataError;

pub mod domain;

pub use domain::clip::field::ShortCode;
pub use domain::clip::ClipError;
pub use domain::clip::Clip;
pub use domain::Time;

pub mod service;

pub use service::ServiceError;
use crate::data::Db;

pub mod web;

use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use web::{renderer::Renderer};

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub db: Db,
}

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<Db>(config.db)
        .manage::<Renderer>(config.renderer)
        .mount("/", web::http::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
}
