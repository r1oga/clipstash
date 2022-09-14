use crate::domain::clip::field::*;
use rocket::form::FromForm;
use serde::Serialize;

#[derive(Debug, Serialize, FromForm)]
pub struct NewClip {
    pub content: Content,
    pub title: Title,
    pub expires: Expires,
    pub password: Password
}

#[derive(Debug, Serialize, FromForm)]
pub struct GetPasswordProtectedClip {
    pub password: Password
}