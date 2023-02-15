use bson::oid::ObjectId;
use rocket::{ Route, serde::json::Json, http::Status };
use crate::{ business::products_business as business, models::dao::product::Product };
use super::validate_request;

#[get("/products")]
async fn get_products() -> Result<Json<Vec<Product>>, (Status, String)> {
  return match
    validate_request(|| async {
      return business::get_products().await;
    }, [].to_vec()).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[get("/products/<oid>")]
async fn get_product(oid: String) -> Result<Json<Option<Product>>, (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  let id = ObjectId::parse_str(oid);
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::get_product(id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[post("/products", data = "<product>")]
async fn post_product(product: Json<Product>) -> Result<Json<ObjectId>, (Status, String)> {
  let inner = product.into_inner();
  return match
    validate_request(|| async { business::insert_product(inner.clone()).await }, [].to_vec()).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[put("/products/<oid>", data = "<product>")]
async fn put_product(oid: String, product: Json<Product>) -> Result<(), (Status, String)> {
  let id = ObjectId::parse_str(oid);
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  let inner = product.into_inner();
  return match
    validate_request(|| async {
      return business::update_product(inner.clone()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

#[delete("/products/<oid>")]
async fn delete_product(oid: String) -> Result<(), (Status, String)> {
  let id = ObjectId::parse_str(oid);
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::delete_product(id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

pub fn get_routes() -> Vec<Route> {
  return routes![get_products, get_product, post_product, put_product, delete_product];
}