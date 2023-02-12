use bson::oid::ObjectId;

use crate::data::products_db as db;
use crate::models::dao::product::Product;

pub async fn get_products() -> Vec<Product> {
  let products = db::get_products().await;
  return match products {
    Err(_) => [].to_vec(),
    Ok(value) => value,
  };
}

pub async fn get_product(oid: ObjectId) -> Option<Product> {
  return db::get_product(oid).await;
}

pub async fn insert_product(product: Product) -> ObjectId {
  let id = db::insert_product(product).await;
  return match id {
    Err(_) => panic!("Unable to insert product."),
    Ok(value) => value  
  };
}

pub async fn update_product(product: Product) {
  db::update_product(product).await;
}

pub async fn delete_product(oid: ObjectId) {
  db::delete_product(oid).await;
}