extern crate core;

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
use crate::domain::maintenance::Maintenance;
use crate::web::hit_counter::HitCounter;

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub db: Db,
    pub hit_counter: HitCounter,
    pub maintenance: Maintenance
}

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<Db>(config.db)
        .manage::<Renderer>(config.renderer)
        .manage::<HitCounter>(config.hit_counter)
        .manage::<Maintenance>(config.maintenance)
        .mount("/", web::http::routes())
        .mount("/api/clip", web::api::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
        .register("/api/clip", web::api::catcher::catchers())
}

#[cfg(test)]
pub mod test {
    pub fn async_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime")
    }
}
