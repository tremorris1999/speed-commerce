use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Product {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub name: Option<String>,
  pub description: Option<String>,
  pub price: Option<f32>,
  pub category: Option<String>,
  pub stock: Option<i32>,
  pub tags: Option<Vec<String>>
}