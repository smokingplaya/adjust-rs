pub mod postgres;
pub mod redis;

// lol
pub type Pool<T> = diesel::r2d2::Pool<T>;

// todo
#[allow(unused)]
pub(crate) type Database<T> = T;