use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(rename = "userName")]
  pub user_name: Option<String>,
  pub email: Option<String>,
  #[serde(rename = "firstName")]
  pub first_name: Option<String>
}