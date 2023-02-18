use bson::{ oid::ObjectId, doc, Uuid };
use futures::TryStreamExt;
use mongodb::{ Collection, error::Error };

use crate::models::dao::review::Review;

use super::get_db_handle;

async fn get_handle() -> Collection<Review> {
  return get_db_handle().await.collection::<Review>("reviews");
}

pub async fn get_reviews(product_id: &ObjectId) -> Result<Vec<Review>, Error> {
  let filter = doc! { "product_id": product_id };
  let handle = get_handle().await;
  let cursor = handle.find(Some(filter), None).await?;
  let vec = cursor.try_collect::<Vec<Review>>().await?;
  return Ok(vec);
}

pub async fn get_review(oid: &ObjectId) -> Result<Option<Review>, Error> {
  let filter = doc! { "_id": oid };
  let handle = get_handle().await;
  return handle.find_one(filter, None).await;
}

pub async fn insert_review(review: &Review) -> Result<ObjectId, Error> {
  let handle = get_handle().await;
  let result = handle.insert_one(review, None).await?;
  let oid = result.inserted_id.as_object_id();
  return match oid {
    Some(oid) => Ok(oid),
    None => panic!("Unable to insert object."),
  };
}

pub async fn update_review(review: &Review) -> Result<(), Error> {
  let filter = doc! { "_id": review.id };
  let update_doc = doc! {
    "$set": {
      "product_id": review.product_id,
      "user": review.user,
      "rating": review.rating,
      "comment": review.comment.clone(),
    }
  };

  let handle = get_handle().await;
  let result = handle.update_one(filter, update_doc, None).await;
  return match result {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

pub async fn delete_review(oid: &ObjectId) -> Result<(), Error> {
  let filter = doc! { "_id": oid };
  let handle = get_handle().await;
  let result = handle.delete_one(filter, None).await;
  return match result {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

pub async fn insert_review_image(review_id: &ObjectId, image_id: &Uuid) -> Result<(), Error> {
  let filter = doc! { "_id": review_id };
  let update_doc = doc! {
    "$push": {
      "images": image_id
    }
  };

  let handle = get_handle().await;
  let result = handle.update_one(filter, update_doc, None).await;
  return match result {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

pub async fn delete_review_image(review_id: &ObjectId, image_id: &Uuid) -> Result<(), Error> {
  let filter = doc! { "_id": review_id };
  let update_doc = doc! {
    "$pull": {
      "images": image_id
    }
  };

  let handle = get_handle().await;
  let result = handle.update_one(filter, update_doc, None).await;
  return match result {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[async_test]
  async fn test_get_reviews() {
    let reviews = get_reviews(&ObjectId::new()).await.expect("Unable to get reviews.");
    assert_eq!(reviews.len() > 0, true);
  }

  #[async_test]
  async fn test_get_review_by_id() {
    let review = get_review(&ObjectId::new()).await.expect("Unable to get review.");
    assert_eq!(review.is_some(), true);
  }

  #[async_test]
  async fn test_insert_review() {
    let review = Review {
      id: None,
      product_id: None,
      user: None,
      rating: None,
      comment: None,
    };

    let result = insert_review(&review).await;
    assert_eq!(result.is_err(), false);
  }

  #[async_test]
  async fn test_update_review() {
    let review = Review {
      id: None,
      product_id: None,
      user: None,
      rating: None,
      comment: None,
    };

    let result = update_review(&review).await;
    assert_eq!(result.is_err(), false);
  }

  #[async_test]
  async fn test_delete_review() {
    let result = delete_review(&ObjectId::new()).await;
    assert_eq!(result.is_err(), false);
  }

  #[async_test]
  async fn test_insert_review_image() {
    let result = insert_review_image(&ObjectId::new(), &Uuid::new()).await;
    assert_eq!(result.is_err(), false);
  }

  #[async_test]
  async fn test_delete_review_image() {
    let review_id = ObjectId::new();
    let image_id = Uuid::new();

    let insert = insert_review_image(&ObjectId::new(), &Uuid::new()).await;
    

    let result = delete_review_image(&ObjectId::new(), &Uuid::new()).await;
    assert_eq!(result.is_err(), false);
  }
}