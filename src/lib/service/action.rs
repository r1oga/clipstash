use crate::data::{query, DbPool, Tx};
use crate::service::ask;
use crate::{Clip, ServiceError, ShortCode};

use std::convert::TryInto;
use crate::data::query::RevocationStatus;
use crate::web::api::ApiKey;

pub async fn get_clip(req: ask::GetClip, pool: &DbPool) -> Result<Clip, ServiceError> {
    let user_password = req.password.clone();
    // From impl converts ask GetClip into data GetClip
    // TryFrom impl converts model Clip result into domain Clip
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

pub async fn new_clip(req: ask::NewClip, pool: &DbPool) -> Result<Clip, ServiceError> {
    Ok(query::new_clip(req, pool).await?.try_into()?)
}

pub async fn update_clip(req: ask::UpdateClip, pool: &DbPool) -> Result<Clip, ServiceError> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}

pub async fn increase_hit_count(shortcode: &ShortCode, hits: u32, pool: &DbPool) -> Result<(), ServiceError>{ Ok(query::increase_hit_count(shortcode, hits, pool).await?) }

pub async fn begin_tx(pool:&DbPool) -> Result<Tx<'_>, ServiceError> { Ok(pool.begin().await?) }

pub async fn end_tx(tx: Tx<'_>) -> Result<(), ServiceError> { Ok(tx.commit().await?) }

pub async fn new_api_key(pool:&DbPool) -> Result<ApiKey, ServiceError> {
    let api_key = ApiKey::default();
    Ok(query::save_api_key(api_key, pool).await?)
}

pub async fn revoke_api_key(api_key:ApiKey, pool:&DbPool) -> Result<RevocationStatus, ServiceError> {
    Ok(query::revoke_api_key(api_key, pool).await?)
}

pub async fn api_key_is_valid(api_key:ApiKey, pool:&DbPool) -> Result<bool, ServiceError> {
    Ok(query::api_key_is_valid(api_key, pool).await?)
}

pub async fn delete_expired(pool: &DbPool) -> Result<u64, ServiceError> {
    Ok(query::delete_expired(pool).await?)
}