use std::sync::Arc;
use controller::test::TestController;
use adjust::{controllers, database::{postgres::Postgres, Pool}, controller::Controller, service::Service};

mod controller;
mod service;
mod models;

#[allow(unused)]
#[derive(Default, Clone)]
pub struct AppState {
  postgres: Arc<Pool<Postgres>>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut service = Service::new("Example");
  service.controllers(controllers![TestController]);

  service.run()
    .await
}