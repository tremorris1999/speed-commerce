use bson::{ doc, oid::ObjectId };
use mongodb::{ Collection, error::Error };

use futures::TryStreamExt;

use crate::models::dao::order::Order;
use super::get_db_handle;

async fn get_handle() -> Collection<Order> {
  return get_db_handle().await.collection::<Order>("orders");
}

pub async fn get_orders_by_user_id(user_id: &ObjectId) -> Result<Vec<Order>, Error> {
  let handle = get_handle().await;
  let result = handle.find(doc! { "userId": user_id }, None).await?;
  let vec = result.try_collect::<Vec<Order>>().await?;
  return Ok(vec);
}

pub async fn get_order_by_id(order_id: &ObjectId) -> Result<Option<Order>, Error> {
  let handle = get_handle().await;
  let result = handle.find_one(doc! { "_id": order_id }, None).await?;
  return Ok(result);
}

pub async fn update_order(order: Order) -> Result<(), Error> {
  let doc =
    doc! {
    "$set": {
      "userId": order.user_id,
      "products": order.products,
      "billingAddress": order.billing_address,
      "mailingAddress": order.mailing_address,
      "subtotal": order.subtotal,
      "shippingCost": order.shipping_cost,
      "total": order.total,
      "tax": order.tax,
      "taxState": order.tax_state,
    }
  };

  let handle = get_handle().await;
  let _result = handle.update_one(doc! { "_id": order.id }, doc, None).await?;
  return Ok(());
}

pub async fn delete_order(id: &ObjectId) -> Result<(), Error> {
  let handle = get_handle().await;
  let result = handle.delete_one(doc! { "_id": id }, None).await;
  return match result {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

pub async fn insert_order(order: &Order) -> Result<ObjectId, Error> {
  let handle = get_handle().await;
  let result = handle.insert_one(order, None).await?;
  let oid = result.inserted_id.as_object_id();
  return match oid {
    Some(oid) => Ok(oid),
    None => panic!("Unable to insert object."),
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::models::dao::order::Order;

  #[async_test]
  async fn test_get_orders_by_user_id() {
    let result = get_orders_by_user_id(&ObjectId::new()).await;
    assert!(result.is_ok());
  }

  #[async_test]
  async fn test_get_order_by_id() {
    let result = get_order_by_id(&ObjectId::new()).await;
    assert!(result.is_ok());
    let order = result.unwrap();
    assert!(order.is_some());
  }

  #[async_test]
  async fn test_update_order() {
    let order = Order {
      id: Some(ObjectId::new()),
      user_id: Some(ObjectId::new()),
      products: Some(vec![]),
      billing_address: None,
      mailing_address: None,
      subtotal: None,
      shipping_cost: None,
      tax: None,
      total: None,
      tax_state: None,
    };

    let result = update_order(order).await;
    assert!(result.is_ok());
  }

  #[async_test]
  async fn test_delete_order() {
    let result = delete_order(&ObjectId::new()).await;
    assert!(result.is_ok());
  }

  #[async_test]
  async fn test_insert_order() {
    let order = Order {
      id: Some(ObjectId::new()),
      user_id: Some(ObjectId::new()),
      products: Some(vec![]),
      billing_address: None,
      mailing_address: None,
      subtotal: None,
      shipping_cost: None,
      tax: None,
      total: None,
      tax_state: None,
    };

    let result = insert_order(&order).await;
    assert!(result.is_ok());
  }
}