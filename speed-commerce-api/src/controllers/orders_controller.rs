use bson::oid::ObjectId;
use rocket::{ Route, serde::json::Json, http::Status };
use crate::{ business::orders_business as business, models::dao::order::Order };
use super::validate_request;

#[get("/orders/user/<oid>")]
async fn get_orders(oid: String) -> Result<Json<Vec<Order>>, (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  let id = ObjectId::parse_str(oid);
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::get_orders(&id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[get("/orders/<oid>")]
async fn get_order(oid: String) -> Result<Json<Option<Order>>, (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  let id = ObjectId::parse_str(oid);
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::get_order_by_id(&id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[post("/orders", data = "<order>")]
async fn post_order(order: Json<Order>) -> Result<Json<ObjectId>, (Status, String)> {
  let inner = order.into_inner();
  return match
    validate_request(|| async { business::insert_order(inner.clone()).await }, [].to_vec()).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[put("/orders/<oid>", data = "<order>")]
async fn put_order(oid: String, order: Json<Order>) -> Result<(), (Status, String)> {
  let id = ObjectId::parse_str(oid);
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  let inner = order.into_inner();
  return match
    validate_request(|| async {
      return business::update_order(inner.clone()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(value),
  };
}

#[delete("/orders/<oid>")]
async fn delete_order(oid: String) -> Result<(), (Status, String)> {
  let id = ObjectId::parse_str(oid);
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::delete_order(&id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(value),
  };
}

pub fn get_routes() -> Vec<Route> {
  return routes![
    get_orders,
    get_order,
    post_order,
    put_order,
    delete_order,
    ]
}