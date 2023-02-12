use rocket::{ Route, Request, Catcher };

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