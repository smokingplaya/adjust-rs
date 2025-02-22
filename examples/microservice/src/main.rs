use controller::test::TestController;
use dixxxie::{
  database::{postgres::{Postgres, PostgresConnection}, Pool},
  server::WebService
};

mod controller;
mod service;
mod models;

#[allow(unused)]
#[derive(Clone)]
struct AppState {
  postgres: Pool<Postgres>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let state = AppState {
    postgres: PostgresConnection::try_connect()?
  };

  WebService::start(state, [TestController], Some(1337))
    .await
}