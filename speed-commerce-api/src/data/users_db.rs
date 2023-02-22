use bson::{doc, oid::ObjectId};
use mongodb::{Collection, error::Error};

use crate::models::{dao::user::User, dto::update_user::UpdateUser};
use super::get_db_handle;

async fn get_handle() -> Collection<User> {
  return get_db_handle().await.collection::<User>("users");
}

pub async fn get_user(user_name: &str) -> Result<Option<User>, Error> {
  let handle = get_handle().await;
  let result = handle.find_one(doc! { "userName": user_name }, None).await?;
  return Ok(result);
}

pub async fn update_user(user: UpdateUser) -> Result<(), Error> {
  let doc = doc! {
    "$set": {
      "userName": user.user_name,
      "email": user.email,
      "firstName": user.first_name,
    }
  };
  let handle = get_handle().await;
  let _result = handle.update_one(doc! { "_id": user.id }, doc, None).await?;
  return Ok(());
}

pub async fn delete_user(id: &ObjectId) -> Result<(), Error> {
  let handle = get_handle().await;
  let result = handle.delete_one(doc! { "_id": id }, None).await;
  return match result {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

pub async fn insert_user(user: &User) -> Result<ObjectId, Error> {
  let handle = get_handle().await;
  let result = handle.insert_one(user, None).await?;
  let oid = result.inserted_id.as_object_id();
  return match oid {
    Some(oid) => Ok(oid),
    None => panic!("Unable to insert object."),
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::models::dao::user::User;

  #[async_test]
  async fn test_get_user() {
    let result = get_user("test").await;
    assert!(result.is_ok());
    let user = result.unwrap();
    assert!(user.is_some());
  }

  #[async_test]
  async fn test_update_user() {
    let id = ObjectId::parse_str("5f9f1b5b9c9d1b0b8c1c1c1c").expect("Unable to parse id");
    let user = UpdateUser {
      id: Some(id),
      user_name: Some("test".to_string()),
      email: Some("test@test.com".to_string()),
      first_name: Some("test".to_string()),
    };

    let result = update_user(user).await;
    assert!(result.is_ok());
  }

  #[async_test]
  async fn test_delete_user() {
    let id = ObjectId::parse_str("5f9f1b5b9c9d1b0b8c1c1c1c").expect("Unable to parse id");
    let result = delete_user(&id).await;
    assert!(result.is_ok());
  }

  #[async_test]
  async fn test_insert_user() {
    let user = User {
      id: None,
      user_name: Some("test".to_string()),
      email: Some("test@test.com".to_string()),
      first_name: Some("test".to_string())
    };

    let result = insert_user(&user).await;
    assert!(result.is_ok());
  }

}