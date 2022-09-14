use crate::domain::clip::field;
use crate::ShortCode;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

// all fields can be public as we already implemented validation in domain
#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: field::Password,
}

impl GetClip {
    fn from_raw(shortcode:&str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: field::Password::default()
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: field::Password::default()
        }
    }
}

impl From<&str> for GetClip {
    fn from(raw: &str) -> Self {
        Self::from_raw(raw)
    }
}