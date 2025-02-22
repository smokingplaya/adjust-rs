use axum::Router;
use crate::controller::{ApplyControllerOnRouter, Controller};

pub struct WebService;

impl WebService {
  /// Example
  ///
  /// ```rs
  /// pub struct AppState {
  ///   postgres: PostgresPool,
  ///   redis: RedisPool,
  /// }
  ///
  /// #[tokio::main]
  /// async fn main() -> anyhow::Result<()> {
  ///   let state = AppState {
  ///     postgres: postgres::connect()?,
  ///     redis: redis::connect()?
  ///   }
  ///
  ///   WebService::start(state, [UserController], None)
  ///       .await
  /// }
  /// ```
  pub async fn start<S, T, const SIZE: usize>(state: S, controllers: [T; SIZE], port: Option<u32>) -> anyhow::Result<()>
  where
    S: Clone + Send + Sync + 'static,
    T: Controller<S>
  {
    if cfg!(debug_assertions) {
      dotenv::dotenv()
        .expect("Unable to find .env file");

      env_logger::init();
    }

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port.unwrap_or(80)))
      .await?;

    let router = Router::new()
      .use_controllers(controllers)
      .with_state(state);

    Ok(axum::serve(listener, router)
      .await?)
  }
}