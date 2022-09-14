use sqlx::Error;
use crate::{ClipError, DataError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("clip error: {0}")]
    Clip(#[from] ClipError),
    #[error("database error: {0}")]
    Data(DataError),
    #[error("not found")]
    NotFound,
    #[error("permissions not met {0}")]
    PermissionError(String),
}

impl From<DataError> for ServiceError {
    fn from(err: DataError) -> Self {
        match err {
            DataError::Database(d) => match d {
                Error::RowNotFound => Self::NotFound,
                other => Self::Data(DataError::Database(other))
            }
        }
    }
}

// for convenience
impl From<Error> for ServiceError {
    fn from(err: Error) -> Self {
        match err {
            Error::RowNotFound => Self::NotFound,
            other=> Self::Data(DataError::Database(other))
        }
    }
}