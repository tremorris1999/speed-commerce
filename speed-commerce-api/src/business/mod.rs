use std::{env, path::{Path}, io::Error};

pub fn ensure_location_exists(location: &String) {
  let path = Path::new(location);
  if !path.exists() {
    std::fs::create_dir_all(path).expect("Unable to create directory!");
  }
}

pub fn write_file(location: &String, data: &[u8]) -> Result<(), Error> {
  return std::fs::write(location, data);
}

pub fn read_file(location: &String) -> Result<Vec<u8>, Error> {
  return std::fs::read(location);
}

pub fn delete_file(location: &String) -> Result<(), Error> {
  return std::fs::remove_file(location);
}

fn get_file_base() -> String {
  let base = env::var("FILE_BASE").expect("FILE_BASE is undefined!");
  let test_mode = env::var("TEST_MODE");
  let mode = match test_mode {
    Ok(_) => "/test/images",
    Err(_) => "/images",
  };

  let mut path = base;
  path.push_str(mode);
  return path;
}

pub mod products_business;
pub mod reviews_business;
pub mod users_business;
pub mod addresses_business;
pub mod orders_business;