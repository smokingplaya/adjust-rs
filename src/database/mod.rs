use r2d2::{ManageConnection, PooledConnection};

pub mod postgres;
pub mod redis;

/// Database pool
///
/// Example:
/// ```rs
/// struct ApplicationState {
///   postgres: Pool<Postgres>,
///   redis: Pool<Redis>
/// }
/// ```
pub type Pool<T> = diesel::r2d2::Pool<T>;
/// Database with estabiled connection
///
/// Example:
/// ```rs
/// #[derive(Serialize)]
/// struct Message {
///   message: String
/// }
///
/// async fn handler(
///   State(state): State<AppState>
/// ) -> HttpResult<Message> {
///   let db: Database<Postgres> = state.postgres.get()?;
///
///   // some logic here
///
///   Ok(Json(Message {
///     message: name
///   }))
/// }
/// ```
pub type Database<T> = PooledConnection<T>;

/// ``PoolBuilder`` - Database pool builder
pub trait PoolBuilder<T: ManageConnection> {
  fn create_pool() -> anyhow::Result<Pool<T>>;
}