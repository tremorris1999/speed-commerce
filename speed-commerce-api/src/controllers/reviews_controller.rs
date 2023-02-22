use bson::{ oid::ObjectId, Uuid };
use rocket::{ Route, serde::json::Json, http::Status };
use crate::{ business::reviews_business as business, models::dao::review::Review };
use super::validate_request;

#[get("/reviews/product/<oid>")]
async fn get_reviews(oid: String) -> Result<Json<Vec<Review>>, (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  let id = ObjectId::parse_str(oid);
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::get_reviews(&id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[get("/reviews/<oid>")]
async fn get_review(oid: String) -> Result<Json<Option<Review>>, (Status, String)> {
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  let id = ObjectId::parse_str(oid);
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::get_review(&id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[post("/reviews", data = "<review>")]
async fn post_review(review: Json<Review>) -> Result<Json<ObjectId>, (Status, String)> {
  let inner = review.into_inner();
  return match
    validate_request(|| async { business::insert_review(&inner.clone()).await }, [].to_vec()).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(Json(value)),
  };
}

#[put("/reviews/<oid>", data = "<review>")]
async fn put_review(oid: String, review: Json<Review>) -> Result<(), (Status, String)> {
  let id = ObjectId::parse_str(oid);
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  let inner = review.into_inner();
  return match
    validate_request(|| async {
      return business::update_review(&inner.clone()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(value),
  };
}

#[delete("/reviews/<oid>")]
async fn delete_review(oid: String) -> Result<(), (Status, String)> {
  let id = ObjectId::parse_str(oid);
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::delete_review(&id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(value) => Ok(value),
  };
}

#[post("/reviews/<oid>/image", data = "<image>")]
async fn post_review_image(oid: String, image: Vec<u8>) -> Result<(), (Status, String)> {
  let id = ObjectId::parse_str(oid);
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }

  return match
    validate_request(|| async {
      return business::insert_review_image(&id.clone().unwrap(), &image.clone()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

#[delete("/reviews/<oid>/image/<image_id>")]
async fn delete_review_image(oid: String, image_id: String) -> Result<(), (Status, String)> {
  let id = ObjectId::parse_str(oid);
  let image_id = Uuid::parse_str(image_id);
  let mut validation_errors: Vec<&'static str> = [].to_vec();
  if id.is_err() {
    validation_errors.push("Unable to parse id.");
  }
  if image_id.is_err() {
    validation_errors.push("Unable to parse image id.");
  }

  return match
    validate_request(|| async {
      return business::delete_review_image(&id.clone().unwrap(), &image_id.clone().unwrap()).await;
    }, validation_errors).await
  {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

pub fn get_routes() -> Vec<Route> {
  routes![
    get_reviews,
    get_review,
    post_review,
    put_review,
    delete_review,
    post_review_image,
    delete_review_image
  ]
}