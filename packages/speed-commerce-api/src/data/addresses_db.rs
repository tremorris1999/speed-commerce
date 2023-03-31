use bson::{ oid::ObjectId, doc };
use futures::TryStreamExt;
use mongodb::{ Collection, error::Error };

use crate::models::dao::address::Address;

use super::get_db_handle;

async fn get_handle() -> Collection<Address> {
  return get_db_handle().await.collection::<Address>("adresses");
}

pub async fn get_addresses_from_user(user_id: &ObjectId) -> Result<Vec<Address>, Error> {
  let handle = get_handle().await;
  let results = handle.find(doc! { "userId": user_id }, None).await?;
  let vec = results.try_collect::<Vec<Address>>().await?;
  return Ok(vec);
}

pub async fn get_address_by_id(address_id: &ObjectId) -> Result<Option<Address>, Error> {
  let handle = get_handle().await;
  let result = handle.find_one(doc! { "_id": address_id }, None).await?;
  return Ok(result);
}

pub async fn delete_address_by_id(address_id: &ObjectId) -> Result<(), Error> {
  let handle = get_handle().await;
  let _result = handle.delete_one(doc! { "_id": address_id }, None).await?;
  return Ok(());
}

pub async fn insert_address(address: &Address) -> Result<ObjectId, Error> {
  let handle = get_handle().await;
  let result = handle.insert_one(address, None).await?;
  return Ok(result.inserted_id.as_object_id().expect("Unable to insert user!"));
}

pub async fn update_address(address: Address) -> Result<(), Error> {
  let doc =
    doc! {
    "$set": {
      "userId": address.user_id,
      "addressType": address.address_type,
      "fullName": address.full_name,
      "address1": address.address1,
      "address2": address.address2,
      "city": address.city,
      "county": address.county,
      "state": address.state,
      "country": address.country,
      "postalCode": address.postal_code
    }
  };

  let handle = get_handle().await;
  let _result = handle.update_one(doc! { "_id": &address.id }, doc, None).await?;
  return Ok(());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[async_test]
  async fn test_insert_address() {
    let address = Address {
      id: None,
      user_id: Some(ObjectId::new()),
      address_type: Some("Billing".to_string()),
      full_name: Some("John Doe".to_string()),
      address1: Some("123 Main St".to_string()),
      address2: Some("Apt 1".to_string()),
      city: Some("New York".to_string()),
      county: Some("New York".to_string()),
      state: Some("NY".to_string()),
      country: Some("USA".to_string()),
      postal_code: Some("10001".to_string()),
    };

    let result = insert_address(&address).await;
    assert!(result.is_ok());
  }

  #[async_test]
  async fn test_get_address_from_user() {
    let user_id = ObjectId::new();
    let result = get_addresses_from_user(&user_id).await;
    assert!(result.is_ok());
  }

  #[async_test]
  async fn test_get_address_by_id() {
    let address_id = ObjectId::new();
    let result = get_address_by_id(&address_id).await;
    assert!(result.is_ok());
  }

  #[async_test]
  async fn test_delete_address() {
    let address_id = ObjectId::new();
    let result = delete_address_by_id(&address_id).await;
    assert!(result.is_ok());
  }

  #[async_test]
  async fn test_update_address() {
    let address = Address {
      id: Some(ObjectId::new()),
      user_id: Some(ObjectId::new()),
      address_type: Some("Billing".to_string()),
      full_name: Some("John Doe".to_string()),
      address1: Some("123 Main St".to_string()),
      address2: Some("Apt 1".to_string()),
      city: Some("New York".to_string()),
      county: Some("New York".to_string()),
      state: Some("NY".to_string()),
      country: Some("USA".to_string()),
      postal_code: Some("10001".to_string()),
    };

    let result = update_address(address).await;
    assert!(result.is_ok());
  }
}