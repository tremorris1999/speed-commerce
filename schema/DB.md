# DB

This document should serve to describe each object type we will be saving in our database

## Product

Generic product type

```ts
{
  _id: ObjectId,
  name: String,
  description: String,
  price: Number,
  image: String,
  category: String,
  stock: Number,
  tags: String[]
}
```

## Review

Reviews are related directly as children of a id-denoted product. Given the one-to-many relationship between products and reviews, I've chosen to separate the two objects.

```ts
{
  _id: ObjectId,
  productId: ObjectId,
  user: ObjectId,
  rating: Number,
  comment: String
}
```

## User

Users are used to represent a user of the application. This will remain seperate from the eventual identity system.

```ts
{
  _id: ObjectId,
  userName: String,
  email: String,
  firstName: String,
}
```

## Address

Addresses are tied to users, but given their one-to-many relationship, I've chosed to keep them separate.

```ts
{
  _id: ObjectId,
  userId: ObjectId,
  addressType: String,  // "mailing" or "billing"
  fullName: String,
  address1: String,
  address2: String,
  city: String,
  county: String,
  state: String,
  country: String,
  postalCode: String
}
```

## Order

Orders are related to users and one of the users addresses, as well as a list of products. Naturally, given the relationships between the three, orders also remain separate.

```ts
{
  _id: ObjectId,
  userId: ObjectId,
  products: ObjectId[]
  mailingAddress: ObjectId,
  billingAddress: ObjectId,
  subtotal: Number,
  shippingCost: Number,
  tax: Number,
  taxState: String,
  total: Number
}
```
