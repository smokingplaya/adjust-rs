use axum::Json;
use adjust::{response::HttpResult, database::{Database, postgres::Postgres}};

use crate::models::hi::MessageDto;

#[derive(Default)]
pub struct TestService;

impl TestService {
  #[allow(unused)]
  pub fn say_hi(
    db: &mut Database<Postgres>,
    name: String
  ) -> HttpResult<MessageDto> {
    // here you can do something with postgres

    Ok(Json(MessageDto {
      message: format!("Hello, {name}")
    }))
  }
}