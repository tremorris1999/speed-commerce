use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(rename = "userId")]
  pub user_id: Option<ObjectId>,
  pub products: Option<Vec<ObjectId>>,
  #[serde(rename = "mailingAddress")]
  pub mailing_address: Option<ObjectId>,
  #[serde(rename = "billingAddress")]
  pub billing_address: Option<ObjectId>,
  pub subtotal: Option<f64>,
  #[serde(rename = "shippingCost")]
  pub shipping_cost: Option<f64>,
  pub tax: Option<f64>,
  #[serde(rename = "taxState")]
  pub tax_state: Option<String>,
  pub total: Option<f64>,
}