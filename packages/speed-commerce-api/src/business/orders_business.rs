use bson::oid::ObjectId;
use rocket::http::Status;

use crate::models::dao::order::Order;
use crate::data::orders_db as db;

pub async fn get_orders(user_id: &ObjectId) -> Result<Vec<Order>, (Status, String)> {
  return match db::get_orders_by_user_id(user_id).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn get_order_by_id(oid: &ObjectId) -> Result<Option<Order>, (Status, String)> {
  return match db::get_order_by_id(&oid).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn insert_order(order: Order) -> Result<ObjectId, (Status, String)> {
  return match db::insert_order(&order).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn update_order(order: Order) -> Result<(), (Status, String)> {
  return match db::update_order(order).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}

pub async fn delete_order(oid: &ObjectId) -> Result<(), (Status, String)> {
  return match db::delete_order(oid).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}



