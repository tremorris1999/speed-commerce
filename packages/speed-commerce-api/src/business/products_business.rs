use bson::oid::ObjectId;
use rocket::http::Status;

use crate::data::products_db as db;
use crate::models::dao::product::Product;

pub async fn get_products() -> Result<Vec<Product>, (Status, String)> {
  return match db::get_products().await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn get_product(oid: ObjectId) -> Result<Option<Product>, (Status, String)> {
  return match db::get_product(oid).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn insert_product(product: Product) -> Result<ObjectId, (Status, String)> {
  return match db::insert_product(product).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn update_product(product: Product) -> Result<(), (Status, String)> {
  return match db::update_product(product).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}

pub async fn delete_product(oid: ObjectId) -> Result<(), (Status, String)> {
  return match db::delete_product(oid).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}