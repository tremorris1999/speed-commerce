use futures::Future;
use rocket::{ Route, Request, Catcher, http::Status };

mod products_controller;

#[catch(400)]
fn bad_request(request: &Request) -> String {
  return request.to_string();
}

#[catch(404)]
fn not_found() {}

#[catch(500)]
fn internal_error() {}

pub fn get_routes() -> Vec<Route> {
  let mut routes = [].to_vec();
  routes.append(&mut products_controller::get_routes());
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
  where F: Fn() -> Fut, Fut: Future<Output = R>
{
  if validation_errors.len() > 0 {
    let mut error_message = "".to_string();
    for error in validation_errors {
      error_message.push_str(error);
      error_message.push_str("\n");
    }
    error_message.pop();
    return Err((Status::BadRequest, error_message));
  }
  else {
    return Ok(func().await);
  }
}