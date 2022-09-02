use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub clip_id: String,
    pub shortcode: String,
    pub content: String,
    pub title: Option<String>,
    pub posted: NaiveDateTime,
    pub expires: NaiveDateTime,
    pub password: Option<String>,
    pub hits: i64,
}

impl TryFrom<Clip> for crate::domain::clip::Clip {
    type Error = ClipError;

    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field::*;
        use std::str::FromStr;

        Ok(Self {
            clip_id: ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
            shortcode: ShortCode::from(clip.shortcode.as_str()),
            content: Content::new(clip.content.as_str())?,
            title: Title::new(clip.title),
            posted: Posted::new(Time::from_naive_utc(clip.posted)),
            expires: Expires::new(Time::from_naive_utc(clip.expires)),
            password: Password::new(clip.password.unwrap_or_default())?,
            hits: Hits::new(u64::try_from(clip.hits)?),
        })
    }
}