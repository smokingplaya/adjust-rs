use std::future::Future;

use crate::{controller::Controller, server::WebService};

/// A ``declarative`` structure for fast web service creation
///
/// # Example
/// ```rs
/// async fn main() -> anyhow::Result<()> {
///   let service = Service {
///     name: "Example",
///     state: ApplicationState {
///       postgres: PostgresConnection::try_connect()?
///     },
///     controllers: [TestController],
///     ..Default::default() // port: None
///   };
///
///   service.run()
///     .await
/// }
/// ```
pub struct Service<'a, S, T, const SIZE: usize>
where
  S: Clone + Send + Sync + 'static,
  T: Controller<S>
{
  pub name: &'a str,
  pub state: S,
  pub controllers: [T; SIZE],
  pub port: Option<u32>
}

impl<S, T, const SIZE: usize> Service<'_, S, T, SIZE>
where
  S: Clone + Send + Sync + 'static,
  T: Controller<S>
{
  pub fn run(self) -> impl Future<Output = anyhow::Result<()>> {
    log::info!("Starting service {}", self.name);

    WebService::start(self.state, self.controllers, self.port)
  }
}
