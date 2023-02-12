use bson::oid::ObjectId;
use rocket::{ Route, serde::json::Json, http::Status };
use crate::{ business::products_business as business, models::dao::product::Product };

#[get("/products")]
async fn get_products() -> Json<Vec<Product>> {
  return Json(business::get_products().await);
}

#[get("/products/<oid>")]
async fn get_product(oid: String) -> Result<Json<Option<Product>>, (Status, &'static str)> {
  let id = match ObjectId::parse_str(oid) {
    Err(_) => None,
    Ok(value) => Some(value),
  };

  return match id {
    None => Err((Status::BadRequest, "Unable to parse id.")),
    Some(value) => Ok(Json(business::get_product(value).await)),
  };
}

#[post("/products", data = "<product>")]
async fn post_product(product: Json<Product>) -> Json<ObjectId> {
  let inner = product.into_inner();
  return Json(business::insert_product(inner).await);
}

#[put("/products/<oid>", data = "<product>")]
async fn put_product(oid: String, product: Json<Product>) -> Result<(), (Status, &'static str)> {
  let id = match ObjectId::parse_str(oid) {
    Err(_) => None,
    Ok(value) => Some(value),
  };

  return match id {
    None => Err((Status::BadRequest, "Unable to parse id.")),
    Some(_value) => {
      let inner = product.into_inner();
      business::update_product(inner).await;
      return Ok(());
    }
  };
}

#[delete("/products/<oid>")]
async fn delete_product(oid: String) -> Result<(), (Status, &'static str)> {
  let id = match ObjectId::parse_str(oid) {
    Err(_) => None,
    Ok(value) => Some(value),
  };

  return match id {
    None => Err((Status::BadRequest, "Unable to parse id.")),
    Some(value) => {
      business::delete_product(value).await;
      return Ok(());
    }
  };
}

pub fn get_routes() -> Vec<Route> {
  return routes![get_products, get_product, post_product, put_product, delete_product];
}