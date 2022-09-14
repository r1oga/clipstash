use crate::domain::clip::field::*;
use crate::ShortCode;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

// all fields can be public as we already implemented validation in domain
#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: Password,
}

impl GetClip {
    fn from_raw(shortcode:&str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: Password::default()
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: Password::default()
        }
    }
}

impl From<&str> for GetClip {
    fn from(raw: &str) -> Self {
        Self::from_raw(raw)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewClip {
    pub content: Content,
    pub title: Title,
    pub expires: Expires,
    pub password: Password,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateClip {
    pub shortcode: ShortCode,
    pub content: Content,
    pub title: Title,
    pub expires: Expires,
    pub password: Password
}