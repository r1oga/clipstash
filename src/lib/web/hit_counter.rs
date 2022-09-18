use std::collections::HashMap;
use crate::data::DbPool;
use crate::ShortCode;
use crate::service::{self, ServiceError};
use crossbeam_channel::TryRecvError;
use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use rocket::async_trait;
use tokio::runtime::Handle;

// mutually exclusive lock (Mutex) shareable between threads (Arc)
type HitStore = Arc<Mutex<HashMap<ShortCode, u32>>>;

#[derive(Debug, thiserror::Error)]
enum HitCountError {
    #[error("service error: {0}")]
    Service(#[from] ServiceError),
    #[error("channel communication error: {0}")]
    Channel(#[from] crossbeam_channel::SendError<HitCountMsg>),
}

enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32),
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>,
}

impl HitCounter {
    fn commit_hits(hits: HitStore, handle: Handle, pool: DbPool) -> Result<(), HitCountError> {
        let hits = Arc::clone(&hits);

        // convert hashmap into vector to avoid compiler errors when passing it
        // as arc mutex to async executor
        let hits: Vec<(ShortCode, u32)> = {
            let mut hits = hits.lock();
            let hits_vec = hits
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect();
            hits.clear();
            hits_vec
        };

        handle.block_on(async move {
            let tx = service::action::begin_tx(&pool).await?;
            for (shortcode, hits) in hits {
                if let Err(e) = service::action::increase_hit_count(&shortcode, hits, &pool).await {
                    eprintln!("error increasing hit count {:?}", e);
                }
            }
            Ok(service::action::end_tx(tx).await?)
        })
    }

    fn process_msg(
        msg: HitCountMsg,
        hits: HitStore,
        handle: Handle, pool:
        DbPool,
    ) -> Result<(), HitCountError> {
        match msg {
            HitCountMsg::Commit => Self::commit_hits(hits.clone(), handle.clone(), pool.clone())?,
            HitCountMsg::Hit(shortcode, count) => {
                let mut hit_count = hits.lock();
                let hit_count = hit_count.entry(shortcode).or_insert(0);
                *hit_count += count;
            }
        }

        Ok(())
    }

    pub fn new(pool: DbPool, handle: Handle) -> Self {
        let (tx, rx) = unbounded();
        let tx_clone = tx.clone();
        let rx_clone = rx.clone();

        let _ = std::thread::spawn(move || {
            println!("HitCounter thread spawned");
            let hits: HitStore = Arc::new(Mutex::new(HashMap::new()));

            loop {
                match rx_clone.try_recv() {
                    Ok(msg) => if let Err(e) = Self::process_msg(
                        msg,
                        hits.clone(),
                        handle.clone(),
                        pool.clone(),
                    ) {
                        eprintln!("message processing error {:?}", e)
                    },
                    Err(e) => match e {
                        // channel empty
                        TryRecvError::Empty => {
                            // wait
                            std::thread::sleep(Duration::from_secs(5));
                            // commit to Db if no errors
                            if let Err(e) = tx_clone.send(HitCountMsg::Commit) {
                                eprintln!("error sending commit msg to hits channel {:?}", e)
                            }
                        },
                        _ => break
                    }
                }
            }
        });

        Self { tx }
    }

    pub fn hit(&self, shortcode: ShortCode, count: u32) {
        if let Err(e) = self.tx.send(HitCountMsg::Hit(shortcode, count)) {
            eprintln!("hit count error {}", e)
        }
    }
}


