use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: u64,
  pub name: String,
  pub occupation: String,
  pub email: String,
  pub phone: String,
}
