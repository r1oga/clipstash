use serde::{Deserialize, Serialize};
use crate::domain::clip::ClipError;
// use super::ClipError;

#[derive(Clone, Debug, Deserialize, Serialize)] // needs for serde because of the JSON API
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    // moving self and return inner value
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}