use serde::{Deserialize, Serialize};
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Hits(u64);

impl Hits {
    // Constructor derived
    // pub fn new(data: u64) -> Self {
    //     Self(data)
    // }

    // moving self and return inner value
    pub fn into_inner(self) -> u64 {
        self.0
    }
}