use diesel::{r2d2::ConnectionManager, PgConnection};
use super::Pool;

pub type Postgres = ConnectionManager<PgConnection>;

pub struct PostgresConnection;

impl PostgresConnection {
  pub fn try_connect() -> anyhow::Result<Pool<Postgres>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Ok(r2d2::Pool::builder().build(manager)?)
  }
}