use super::Pool;

pub type Redis = redis::Client;

pub struct RedisConnection;

impl RedisConnection {
  pub fn new() -> anyhow::Result<Pool<Redis>> {
    let redis_url = std::env::var("REDIS_URL")?;
    let manager = redis::Client::open(redis_url)?;

    Ok(Pool::builder().build(manager)?)
  }
}