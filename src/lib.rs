pub mod controller;
pub mod response;
pub mod connection;

pub use anyhow;
pub use axum;
pub use serde;
pub use diesel;

pub fn setup() {
  if cfg!(debug_assertions) {
    dotenv::dotenv().ok();
  }

  env_logger::init();
}