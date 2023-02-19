use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Review {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(rename = "productId")]
  pub product_id: Option<ObjectId>,
  pub user: Option<ObjectId>,
  pub rating: Option<f32>,
  pub comment: Option<String>
}