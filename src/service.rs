use std::future::Future;
use crate::{controller::ControllerList, server::WebServer};

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
#[derive(Default)]
pub struct Service<'a, S>
where
  S: Clone + Send + Sync + 'static,
{
  pub name: &'a str,
  pub state: S,
  pub controllers: ControllerList<S>,
  pub port: Option<u32>
}

impl<S> Service<'_, S>
where
  S: Clone + Send + Sync + 'static,
{
  pub fn run(self) -> impl Future<Output = anyhow::Result<()>> {
    log::info!("Starting service {} (v{})", self.name, env!("CARGO_PKG_VERSION"));

    WebServer::start(self.state, self.controllers, self.port)
  }
}
