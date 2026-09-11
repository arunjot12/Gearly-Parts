# Gearly-Product

This is the product microservice for Gearly.

## Current Progress

- **Framework**: Rust with Axum and Tokio.
- **Database**: MySQL using Diesel ORM and deadpool-diesel for connection pooling.
- **Authentication**: JWT-based authentication middleware is implemented and applied to routes.

### Implemented Endpoints

- `POST /create_product` - Create a new product. Expects a JSON payload (`name`, `price`, `descri`, `part_number`). Validates that the price is greater than 0.
- `GET /get_product` - Retrieve a specific product by ID (passed in the JSON payload).
- `GET /get_products` - Retrieve all products.
- `POST /delete_product` - Delete a product by its ID (passed in the JSON payload).

### Models

The `Product` model includes the following fields:
- `id`
- `name`
- `price`
- `descri`
- `part_number`
- `created_at`
- `updated_at`
