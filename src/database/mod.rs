use r2d2::PooledConnection;

pub mod postgres;
pub mod redis;

pub type Pool<T> = diesel::r2d2::Pool<T>;
pub type Database<T> = PooledConnection<T>;