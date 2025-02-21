pub mod postgres;
pub mod redis;

// lol
pub(crate) type Pool<T> = diesel::r2d2::Pool<T>;

// todo
#[allow(unused)]
pub(crate) type Database<T> = T;