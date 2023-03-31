use bson::oid::ObjectId;
use rocket::http::Status;

use crate::models::dao::address::Address;
use crate::data::addresses_db as db;

pub async fn get_addresses(user_id: &ObjectId) -> Result<Vec<Address>, (Status, String)> {
  return match db::get_addresses_from_user(user_id).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn get_address(oid: &ObjectId) -> Result<Option<Address>, (Status, String)> {
  return match db::get_address_by_id(&oid).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn insert_address(address: Address) -> Result<ObjectId, (Status, String)> {
  return match db::insert_address(&address).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn update_address(address: Address) -> Result<(), (Status, String)> {
  return match db::update_address(address).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}

pub async fn delete_address(oid: &ObjectId) -> Result<(), (Status, String)> {
  return match db::delete_address_by_id(oid).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}