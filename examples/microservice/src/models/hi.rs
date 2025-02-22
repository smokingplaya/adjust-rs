use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MessageDto {
  pub message: String
}