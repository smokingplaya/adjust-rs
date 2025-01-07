/// Database (PostgreSQL) connection

use std::env;
use anyhow::Result;
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Result<DbPool> {
  let database_url = env::var("DATABASE_URL")?;
  let manager = ConnectionManager::<PgConnection>::new(database_url);

  Ok(r2d2::Pool::builder().build(manager)?)
}