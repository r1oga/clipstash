use crate::data::{query, DbPool, Tx};
use crate::service::ask;
use crate::{Clip, ServiceError, ShortCode};

use std::convert::TryInto;

pub async fn get_clip(req: ask::GetClip, pool: &DbPool) -> Result<Clip, ServiceError> {
    let user_password = req.password.clone();
    // From impl convert ask GetClip into data GetClip
    // TryFrom impl convert model Clip result into domain Clip
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;

    if clip.password.has_password() {
        if clip.password == user_password {
            Ok(clip)
        } else {
            Err(ServiceError::PermissionError("Invalid password".to_owned()))
        }
    } else {
        Ok(clip)
    }
}