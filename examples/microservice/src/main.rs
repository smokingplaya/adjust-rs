#[tokio::main]
async fn main() -> Result<()> {
  let state = AppState {
    postgres: establish_connection()?
  };

  let router = Router::new()
    .apply_controller(BalanceController)
    .with_state(state);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
    .await?;

  Ok(axum::serve(listener, router).await?)
}