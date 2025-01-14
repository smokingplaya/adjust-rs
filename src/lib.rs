pub mod controller;
pub mod response;
pub mod connection;

pub use axum;
pub use serde;
pub use diesel;

pub fn setup() -> anyhow::Result<()> {
  if cfg!(debug_assertions) {
    dotenv::dotenv()?;
  }

  env_logger::init();

  Ok(())
}