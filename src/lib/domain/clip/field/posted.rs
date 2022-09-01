use crate::domain::clip::ClipError;
use crate::domain::Time;
// use super::ClipError;
use derive_more::Constructor;
use crate::domain::time::Time;

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}