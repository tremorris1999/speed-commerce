use bson::oid::ObjectId;
use rocket::Route;
use rocket::{ serde::json::Json, http::Status };

use crate::models::dao::user::User;
use crate::business::users_business as business;
use crate::models::dto::update_user::UpdateUser;

use super::validate_request;

#[get("/users/<user_name>")]
async fn get_user_by_user_name(user_name: String) -> Result<Json<User>, (Status, String)> {
  return match
    validate_request(|| async {
      return business::get_user(user_name.clone()).await;
    }, [].to_vec()).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value.unwrap())),
  };
}

#[put("/users/<id>", data = "<user>")]
async fn update_user(id: String, user: Json<UpdateUser>) -> Result<(), (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();

  let id = ObjectId::parse_str(id);
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  let inner = user.into_inner();
  return match
    validate_request(|| async {
      return business::update_user(id.clone().unwrap(), inner.clone()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

#[delete("/users/<id>")]
async fn delete_user(id: String) -> Result<(), (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();

  let oid = ObjectId::parse_str(id);
  if oid.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::delete_user(&oid.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(value),
  };
}

#[post("/users", data = "<user>")]
async fn post_user(user: Json<User>) -> Result<Json<ObjectId>, (Status, String)> {
  let inner = user.into_inner();
  return match
    validate_request(|| async { business::create_user(&inner.clone()).await }, [].to_vec()).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

pub fn get_routes() -> Vec<Route> {
  routes![get_user_by_user_name, update_user, delete_user, post_user]
}