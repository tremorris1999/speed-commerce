use bson::Uuid;
use bson::oid::ObjectId;
use rocket::http::Status;

use crate::data::reviews_db as db;
use crate::models::dao::review::Review;

use super::{ get_file_base, ensure_location_exists, write_file, delete_file };

pub async fn get_reviews(product_id: &ObjectId) -> Result<Vec<Review>, (Status, String)> {
  return match db::get_reviews(product_id).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn get_review(oid: &ObjectId) -> Result<Option<Review>, (Status, String)> {
  return match db::get_review(oid).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn insert_review(review: &Review) -> Result<ObjectId, (Status, String)> {
  return match db::insert_review(review).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(value) => Ok(value),
  };
}

pub async fn update_review(review: &Review) -> Result<(), (Status, String)> {
  return match db::update_review(review).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}

pub async fn delete_review(oid: &ObjectId) -> Result<(), (Status, String)> {
  return match db::delete_review(oid).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}

pub async fn insert_review_image(
  review_id: &ObjectId,
  image: &Vec<u8>
) -> Result<(), (Status, String)> {
  let image_id = save_review_image_file(&image);
  if image_id.is_err() {
    return Err((Status::InternalServerError, image_id.err().unwrap().to_string()));
  }

  return match db::insert_review_image(review_id, &image_id.unwrap()).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}

pub async fn delete_review_image(
  review_id: &ObjectId,
  image_id: &Uuid
) -> Result<(), (Status, String)> {
  let result = delete_review_image_file(image_id);
  if result.is_err() {
    return Err((Status::InternalServerError, result.err().unwrap().to_string()));
  }

  return match db::delete_review_image(review_id, image_id).await {
    Err(value) => Err((Status::InternalServerError, value.to_string())),
    Ok(_) => Ok(()),
  };
}

fn save_review_image_file(bytes: &Vec<u8>) -> Result<Uuid, String> {
  let image_id = Uuid::new();
  let mut path = get_file_base();
  path.push_str("/reviews/images");
  ensure_location_exists(&path);
  let string_id = format!("/{}", image_id.to_string());
  path.push_str(&string_id);
  let result = write_file(&path, bytes);
  return match result {
    Err(value) => Err(value.to_string()),
    Ok(_) => Ok(image_id),
  };
}

fn delete_review_image_file(image_id: &Uuid) -> Result<(), String> {
  let mut path = get_file_base();
  path.push_str("/reviews/images");
  ensure_location_exists(&path);
  let string_id = format!("/{}", image_id.to_string());
  path.push_str(&string_id);
  let result = delete_file(&path);
  return match result {
    Err(value) => Err(value.to_string()),
    Ok(_) => Ok(()),
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_save_review_image_file() {
    let bytes = [].to_vec();
    let result = save_review_image_file(&bytes);
    assert!(result.is_ok());

    let image_id = result.unwrap();
    _ = delete_review_image_file(&image_id);
  }

  #[test]
  fn test_delete_review_image_file() {
    let bytes = [].to_vec();
    let save = save_review_image_file(&bytes);

    let image_id = save.unwrap();
    let result = delete_review_image_file(&image_id);
    assert!(result.is_ok());
  }
}