use controllers::{get_routes, get_catchers};
use rocket::fairing::{ Fairing, Info, Kind };
use rocket::http::Header;
use rocket::{ Request, Response };

mod controllers;
pub mod data;
pub mod business;
pub mod models;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
  rocket::build().register("/", get_catchers()).attach(CORS).mount("/", get_routes())
}

pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
  fn info(&self) -> Info {
    Info {
      name: "Attaching CORS headers to responses",
      kind: Kind::Response,
    }
  }

  async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
    response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
    response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
  }
}