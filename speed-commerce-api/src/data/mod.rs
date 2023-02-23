use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::{ Database, Client, options::{ ClientOptions } };
use std::env;

lazy_static! {
  pub static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
    return get_client_handle().await;
  });
}

async fn get_client_handle() -> Client {
  let mongo_url = env::var("MONGO_URI").expect("MONGO_URI is undefined!");
  let mongo_db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is undefined!");
  let mut options = ClientOptions::parse(&mongo_url).await.expect("Unable to parse MONGO_URI!");
  options.max_pool_size = Some(100);
  options.min_pool_size = Some(10);
  options.max_idle_time = Some(std::time::Duration::from_secs(300));
  options.default_database = Some(mongo_db_name);
  let client = Client::with_options(options).expect("Unable to create client!");
  return client;
}

pub async fn get_db_handle() -> Database {
  let test_mode = env::var("TEST_MODE");
  return match test_mode {
    Ok(_) => get_client_handle().await.default_database().expect("Unable to get default database!"),
    Err(_) => CLIENT.get().await.default_database().expect("Unable to get default database!")
  };
}

pub mod products_db;
pub mod reviews_db;
pub mod users_db;
pub mod addresses_db;