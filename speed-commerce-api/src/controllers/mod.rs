use futures::Future;
use rocket::{ Route, Request, Catcher, http::Status };

#[catch(400)]
fn bad_request(request: &Request) -> String {
  return request.to_string();
}

#[catch(404)]
fn not_found() {}

#[catch(500)]
fn internal_error(request: &Request) -> String {
  return request.to_string();
}

pub fn get_routes() -> Vec<Route> {
  let mut routes = [].to_vec();
  routes.append(&mut products_controller::get_routes());
  routes.append(&mut reviews_controller::get_routes());
  routes.append(&mut users_controller::get_routes());
  return routes;
}

pub fn get_catchers() -> Vec<Catcher> {
  return catchers![bad_request, not_found, internal_error];
}

pub async fn validate_request<F, Fut, R>(
  func: F,
  validation_errors: Vec<&'static str>
)
  -> Result<R, (Status, String)>
  where F: Fn() -> Fut, Fut: Future<Output = Result<R, (Status, String)>>
{
  if validation_errors.len() > 0 {
    let mut error_message = "".to_string();
    for error in validation_errors {
      error_message.push_str(error);
      error_message.push_str("\n");
    }
    error_message.pop();
    return Err((Status::BadRequest, error_message)); // 400 if request is invalid.
  } else {
    return match func().await {
      Err(value) => Err(value), // 500 if function fails.
      Ok(value) => Ok(value),
    };
  }
}

mod products_controller;
mod reviews_controller;
mod users_controller;