# Back

This document should serve to document the intended design of our backend architecture.

## Data

This layer serves as our link to our database. The intended flow is to query the database and return our result set with maximum filtering done by the optimized database filtering system while maintaining transparent business logic and control flow.

Example:

```rust
// let coll be an arbitrary collection from our database

// Avoid this ❌
let items = coll.find(None, None).await
  .try_collect::<Vec<Product>>().await
  .into_iter()
  .filter(|item| item.count < 10)
  .collect::<Vec<MyItem>>();

// Do this ✅
let items = products.find(doc! { "count": { "$lt": 10 } }, None).await
  .try_collect::<Vec<MyItem>>().await;
```

## Business

This layer serves as a business logic processing layer beyond simple db filtering. This should be used for things like populating items using another db call.

Example:

```rust
use data::items;

fn foo() -> MyItem {
  let item = getItem("id"); // goes to data layer
  item.subitems = getSubItems("id"); // goes to data layer
  return item;
}
```

## API

This layer will serve our api functionality to the front end.

Each controller will implement several routes/actions as needed as well as the `routes` function as follows:

```rust
#[get("/controller/action")]
async fn foo() -> &'static str {
  "bar"
}

/**etc */

fn routes() -> Vec<Route> {
  return !routes[foo, /**etc */];
}
```

These will be aggregated in the controllers module into a module-wide `routes` function to be bound in our entry point as:

```rust
#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", controllers::routes())
}
```

## Models

Our models module can be separated into two categories.

### DAO

Our DAOs will serve for working with our database objects and dealing with business logic. Some DAO's may be used to send through the wire when appropriate.

### DTO

Our DTOs will serve to transfer ONLY relevant data through the wire. These should derive from a subset of out DAOs.
