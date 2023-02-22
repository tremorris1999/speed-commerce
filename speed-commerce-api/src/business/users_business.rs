use bson::oid::ObjectId;
use rocket::http::Status;

use crate::models::dao::user::User;
use crate::data::users_db as db;
use crate::models::dto::update_user::UpdateUser;

pub async fn get_user(
  id: Option<&ObjectId>,
  user_name: Option<&str>,
  email: Option<&str>
) -> Result<Option<User>, (Status, String)> {
  let mut user: Option<User> = None;
  if id.is_some() {
    let result = db::get_user_by_id(id.unwrap()).await;
    user = match result {
      Ok(value) => value,
      Err(_) => None,
    };
  }

  if user_name.is_some() && user.is_none() {
    let result = db::get_user_by_user_name(user_name.unwrap()).await;
    user = match result {
      Ok(value) => value,
      Err(_) => None,
    };
  }

  if email.is_some() && user.is_none() {
    let result = db::get_user_by_email(email.unwrap()).await;
    user = match result {
      Ok(value) => value,
      Err(_) => None,
    };
  }

  return Ok(user);
}

pub async fn update_user(id: ObjectId, user: UpdateUser) -> Result<(), (Status, String)> {
  let is_mismatch = user.id.is_some() && user.id.unwrap() != id;
  if is_mismatch {
    return Err((Status::BadRequest, "Unable to update mismatched user".to_string()));
  }

  let result = db::update_user(user).await;
  return match result {
    Ok(_) => Ok(()),
    Err(_) => Err((Status::InternalServerError, "Unable to update user!".to_string())),
  };
}

pub async fn delete_user(id: &ObjectId) -> Result<(), (Status, String)> {
  let result = db::delete_user(id).await;
  return match result {
    Ok(_) => Ok(()),
    Err(_) => Err((Status::InternalServerError, "Unable to delete user!".to_string())),
  };
}

pub async fn create_user(user: &User) -> Result<ObjectId, (Status, String)> {
  let result = db::insert_user(user).await;
  return match result {
    Ok(value) => Ok(value),
    Err(_) => Err((Status::InternalServerError, "Unable to create user!".to_string())),
  };
}