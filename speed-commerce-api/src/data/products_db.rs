use crate::{ models::dao::product::Product, data::get_db_handle };
use bson::{ oid::ObjectId, Document, doc };
use futures::TryStreamExt;
use mongodb::{ Collection, error::Error };

async fn get_handle() -> Collection<Product> {
  return get_db_handle().await.collection::<Product>("products");
}

async fn get_products_with_filter(filter: Option<Document>) -> Result<Vec<Product>, Error> {
  let handle = get_handle().await;
  let results = handle.find(filter, None).await?;
  let vec = results.try_collect::<Vec<Product>>().await?;
  return Ok(vec);
}

pub async fn get_products() -> Result<Vec<Product>, Error> {
  return Ok(get_products_with_filter(None).await?);
}

pub async fn get_product(oid: ObjectId) -> Option<Product> {
  let handle = get_handle().await;
  let result = handle.find_one(doc! { "_id": oid }, None).await;
  return match result {
    Err(_) => None,
    Ok(value) => value,
  };
}

pub async fn insert_product(product: Product) -> Result<ObjectId, Error> {
  let handle = get_handle().await;
  let result = handle.insert_one(product, None).await?;
  return match result.inserted_id.as_object_id() {
    None => panic!("Unable to insert object."),
    Some(value) => Ok(value),
  };
}

pub async fn update_product(product: Product) {
  let update_doc =
    doc! {
    "$set": {
      "_id": product.id,
      "name": product.name,
      "description": product.description,
      "price": product.price,
      "category": product.category,
      "stock": product.stock,
      "tags": product.tags
    }
  };

  let handle = get_handle().await;
  _ = handle.update_one(doc! { "_id": product.id }, update_doc, None).await;
}

pub async fn delete_product(oid: ObjectId) {
  let handle = get_handle().await;
  _ = handle.delete_one(doc! { "_id": oid }, None).await;
}

#[cfg(test)]
mod test {
  use super::*;

  #[async_test]
  async fn test_get_handle() {
    let handle = get_handle().await;
    assert_eq!(handle.name(), "products");
  }

  #[async_test]
  async fn test_get_products_with_filter() {
    let filter = doc! { "name": "Test Item 2" };
    let products = get_products_with_filter(Some(filter)).await.expect("Unable to get products with filter.");
    assert_eq!(products.len(), 1);
  }

  #[async_test]
  async fn test_get_products() {
    let products = get_products().await.expect("Unable to get products.");
    assert_eq!(products.len() > 0, true);
  }

  #[async_test]
  async fn test_get_product() {
    let products = get_products().await.expect("Unable to get products.");
    let product = get_product(products[0].id.expect("No ID available")).await;
    assert_eq!(product.is_some(), true);
  }

  #[async_test]
  async fn test_insert_product() {
    let product = Product {
      id: None,
      name: Some("Test Item 3".to_string()),
      description: Some("Test Item 3 Description".to_string()),
      price: Some(3.99),
      category: Some("Test".to_string()),
      stock: Some(10),
      tags: Some(vec!["test".to_string(), "item".to_string()]),
    };
    let oid = insert_product(product).await;
    assert_eq!(oid.is_ok(), true);
  }

  #[async_test]
  async fn test_update_product() {
    let products = get_products().await.expect("Unable to get products.");
    let mut product = products[0].clone();
    product.name = Some("Test Item 2".to_string());
    update_product(product).await;
    let product = get_product(products[0].id.expect("No ID available")).await;
    assert_eq!(product.is_some(), true);
    assert_eq!(product.unwrap().name, Some("Test Item 2".to_string()));
  }

  #[async_test]
  async fn test_delete_product() {
    let product = Product {
      id: None,
      name: Some("DeleteMe".to_string()),
      description: Some("test".to_string()),
      price: Some(3.99),
      category: Some("Test".to_string()),
      stock: Some(10),
      tags: Some(vec!["test".to_string(), "item".to_string()]),
    };
    let oid = insert_product(product).await.expect("Unable to insert product.");
    delete_product(oid).await;
    let product = get_product(oid).await;
    assert_eq!(product.is_none(), true);
  }
  
}