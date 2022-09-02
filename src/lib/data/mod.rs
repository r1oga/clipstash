pub mod model;

use serde::{Deserialize, Serialize};
use derive_more::{Display, From};
use uuid::Uuid;
use std::str::FromStr;
use sqlx::{Sqlite};

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error)
}

pub type Db = Database<Sqlite>;
pub type DbPool = sqlx::sqlite::SqlitePool; // pool of connections. Reuse already open connections for better perf
pub type Tx<'t> = sqlx::Transaction<'t, Sqlite>; // to allow rolling back
pub type DbRow = sqlx::sqlite::SqliteRow;
pub type QueryResult = sqlx::sqlite::SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);
impl Database<Sqlite> {
  pub async fn new(db_url:&str) -> Self {
      let pool = sqlx::sqlite::SqlitePoolOptions::new().connect(db_url).await;
      match pool {
          Ok(pool) => Self(pool),
          Err(e) => {
              eprintln!("{:?}\n", e);
              eprintln!("If the database has not yet been created, run:\n$sqlx database setup");
              panic!("database connection error")
          }
      }
  }

    pub fn get_pool(&self) -> &DbPool{ &self.0 }
}

#[derive(Clone, Debug, Display, From, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(s)?))
    }
}