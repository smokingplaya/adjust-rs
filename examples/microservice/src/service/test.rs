use axum::Json;
use dixxxie::response::HttpResult;

use crate::models::hi::MessageDto;

pub struct TestService;

impl TestService {
  pub fn say_hi(name: String) -> HttpResult<MessageDto> {
    Ok(Json(MessageDto {
      message: format!("Hello, {name}")
    }))
  }
}