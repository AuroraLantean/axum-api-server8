use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct User {
  pub id: i32, //u64 is not supported in sqlx
  pub name: String,
  pub password: String,
  pub occupation: String,
  pub email: String,
  pub phone: String,
  //pub balance: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct UserList {
  list: Vec<User>,
}
