use crate::data::DbPool;
use crate::ShortCode;
use crate::service::{self, ServiceError};
use crossbeam_channel::TryRecvError;
use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;

enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32)
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>
}

impl HitCounter {
    pub fn new(pool: DbPool, handle: Handle) -> Self {
        todo!()
    }
}


