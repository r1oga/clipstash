use handlebars::RenderError;

pub mod ctx;
pub mod renderer;
pub mod form;
pub mod http;
pub mod hit_counter;

pub use hit_counter::HitCounter;

pub mod api;

pub const PASSWORD_COOKIE: &str = "password";

#[derive(rocket::Responder)]
pub enum PageError {
    #[response(status = 500)]
    Serialization(String),
    #[response(status = 500)]
    Render(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    Internal(String),
}

// thiserror::Error not compatible with Responder, need to impl From manually
impl From<RenderError> for PageError {
    fn from(err: RenderError) -> Self {
        PageError::Render(format!("{}", err))
    }
}

impl From<serde_json::Error> for PageError {
    fn from(err: serde_json::Error) -> Self {
        PageError::Serialization(format!("{}", err))
    }
}

#[cfg(test)]
pub mod test {
    use crate::test::async_runtime;
    use crate::{Renderer, RocketConfig};
    use rocket::local::blocking::Client;

    pub fn config() -> RocketConfig {
        use crate::web::{hit_counter::HitCounter, renderer::Renderer};

        let rt = async_runtime();
        let renderer = Renderer::new("templates/".into());
        let db = crate::data::test::new_db(rt.handle());
        let maintenance = crate::domain::maintenance::Maintenance::spawn(
            db.get_pool().clone(),
            rt.handle().clone());
        let hit_counter = HitCounter::new(db.get_pool().clone(), rt.handle().clone());

        RocketConfig {
            renderer,
            db,
            hit_counter,
            maintenance
        }
    }

    pub fn client() -> Client {
        let config = config();
        Client::tracked(crate::rocket(config)).expect("failed to build rocket instance")
    }
}