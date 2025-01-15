use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Serialize)]
pub struct HttpMessage {
  pub message: String,
}

#[derive(Debug)]
pub struct HttpError(pub String, pub Option<StatusCode>);

pub type HttpResult<T> = core::result::Result<T, HttpError>;

impl HttpError {
  pub fn new(msg: &str) -> HttpError {
    HttpError(msg.to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR))
  }

  pub fn new_with_code(
    msg: &str,
    code: StatusCode
  ) -> HttpError {
    HttpError(msg.to_string(), Some(code))
  }
}

impl IntoResponse for HttpError {
  fn into_response(self) -> axum::response::Response {
    (
      self.1.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
      Json(HttpMessage {
        message: self.0.to_string(),
      }),
    )
        .into_response()
  }
}

impl Display for HttpError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Error for HttpError {}

impl From<anyhow::Error> for HttpError {
  fn from(error: anyhow::Error) -> Self {
    HttpError(error.to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR))
  }
}