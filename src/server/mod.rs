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
  ///   let router = WebService::create_router(state, [UserController]);
  ///
  ///   WebService::start(router, None)
  ///       .await
  /// }
  /// ```
  pub async fn start(router: Router, port: Option<u32>) -> std::io::Result<()> {
    if cfg!(debug_assertions) {
      dotenv::dotenv()
        .expect("Unable to find .env file");

      env_logger::init();
    }

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port.unwrap_or(80)))
      .await?;

    axum::serve(listener, router)
      .await
  }

  pub fn create_router<S, T: Controller<S> + Clone, const SIZE: usize>(
    state: S,
    controllers: [T; SIZE]
  ) -> Router<S>
  where
    S: Clone + Send + Sync + 'static
  {
    let router = Router::new()
      .use_controllers(controllers)
      .with_state(state);

    router
  }
}