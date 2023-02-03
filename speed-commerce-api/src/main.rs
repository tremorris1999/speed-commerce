use rocket::fairing::{ Fairing, Info, Kind };
use rocket::http::Header;
use rocket::{ Request, Response };


mod data;
mod models;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
  let mut routes = [].to_vec();
  // TODO: add routes from future controllers module
  rocket::build().attach(CORS).mount("/", routes)
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