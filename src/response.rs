use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug)]
pub struct HttpError(pub anyhow::Error, pub Option<StatusCode>);

pub type HttpResult<T> = core::result::Result<T, HttpError>;

impl HttpError {
  pub fn new(
    message: &'static str,
    code: Option<StatusCode>
  ) -> Self {
    HttpError(anyhow::anyhow!(message), code)
  }
}

#[derive(Serialize)]
struct HttpErrorMessage {
  is_error: bool,
  message: String
}

#[derive(Serialize)]
pub struct HttpMessage {
  pub message: String
}

impl HttpMessage {
  pub fn new(message: &str) -> Self {
    HttpMessage {
      message: message.to_string()
    }
  }
}

impl<E> From<E> for HttpError
where
  E: Into<anyhow::Error> + 'static
{
  fn from(err: E) -> Self {
    HttpError(err.into(), Some(StatusCode::INTERNAL_SERVER_ERROR))
  }
}

impl IntoResponse for HttpError {
  fn into_response(self) -> axum::response::Response {
    (
      self.1.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
      Json(HttpErrorMessage {
        is_error: true,
        message: self.0.to_string()
      })
    ).into_response()
  }
}