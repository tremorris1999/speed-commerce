use bson::oid::ObjectId;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::models::dao::address::Address;
use crate::business::addresses_business as business;
use super::validate_request;

#[get("/addresses/user/<user_id>")]
async fn get_addresses(user_id: String) -> Result<Json<Vec<Address>>, (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  let id = ObjectId::parse_str(user_id);
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::get_addresses(&id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[get("/addresses/<oid>")]
async fn get_address(oid: String) -> Result<Json<Option<Address>>, (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  let id = ObjectId::parse_str(oid);
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::get_address(&id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[post("/addresses", data = "<address>")]
async fn post_address(address: Json<Address>) -> Result<Json<ObjectId>, (Status, String)> {
  let inner = address.into_inner();
  return match
    validate_request(|| async { business::insert_address(inner.clone()).await }, [].to_vec()).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[put("/addresses/<oid>", data = "<address>")]
async fn put_address(oid: String, address: Json<Address>) -> Result<(), (Status, String)> {
  let id = ObjectId::parse_str(oid);
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  let inner = address.into_inner();
  return match
    validate_request(|| async {
      return business::update_address(inner.clone()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(value),
  };
}

#[delete("/addresses/<oid>")]
async fn delete_address(oid: String) -> Result<(), (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  let id = ObjectId::parse_str(oid);
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::delete_address(&id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(value),
  };
}

pub fn get_routes() -> Vec<rocket::Route> {
  return routes![get_addresses, get_address, post_address, put_address, delete_address];
}