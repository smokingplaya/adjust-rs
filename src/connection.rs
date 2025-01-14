use std::env;
use anyhow::Result;
use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection}, PgConnection};

// PostgreSQL connection
type Postgres = ConnectionManager<PgConnection>;
pub type DbPool = diesel::r2d2::Pool<Postgres>;
pub type DbPooled = PooledConnection<Postgres>;

pub fn establish_connection() -> Result<DbPool> {
  let database_url = env::var("DATABASE_URL")?;
  let manager = ConnectionManager::<PgConnection>::new(database_url);

  Ok(r2d2::Pool::builder().build(manager)?)
}

// Redis connection
pub type RedisPool = Pool<redis::Client>;
pub type RedisPooled = PooledConnection<redis::Client>;

pub fn establish_redis_connection() -> Result<RedisPool> {
  let redis_url = env::var("REDIS_URL")?;
  let manager = redis::Client::open(redis_url)?;

  Ok(Pool::builder().build(manager)?)
}