use crate::data::DbPool;
use crate::service;
use std::time::Duration;
use tokio::runtime::Handle;

pub struct Maintenance;

impl Maintenance {
    pub fn spawn(pool: DbPool, handle: Handle) -> Self {
        // do not block
        handle.spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;

                if let Err(e) = service::action::delete_expired(&pool).await {
                    eprintln!("failed to deleted expired clips: {:?}", e);
                }
            }
        });
        Self
    }
}