use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateUser {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
  pub user_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
  pub first_name: Option<String>,
}