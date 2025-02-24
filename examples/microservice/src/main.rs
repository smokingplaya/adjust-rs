use std::sync::Arc;
use controller::test::TestController;
use adjust::{main, controllers, database::{postgres::Postgres, Pool}, controller::Controller, service::Service};

mod controller;
mod service;
mod models;

#[allow(unused)]
#[derive(Default, Clone)]
pub struct AppState {
  postgres: Arc<Pool<Postgres>>
}

#[main]
async fn main() -> Service<'static, AppState> {
  Service {
    name: "Example",
    state: AppState::default(),
    controllers: controllers![TestController],
    ..Default::default() // port: None
  }
}