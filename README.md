# Order Management API

A RESTful API for managing customer orders built with Rust and Actix-web.

## Features

- Create, read, update, and delete orders
- Update order status
- In-memory storage using HashMap
- Comprehensive error handling
- Full test coverage
- UUID-based order IDs
- Timestamp tracking for created/updated times

## Requirements

- Rust 1.70 or higher
- Cargo

## Setup and Installation

1. Clone or extract the project
2. Navigate to the project directory:
```bash
cd order-api
```

3. Build the project:
```bash
cargo build
```

4. Run the server:
```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

## Running Tests

Execute all tests with:
```bash
cargo test
```

For verbose output:
```bash
cargo test -- --nocapture
```

## API Endpoints

### 1. Create Order
- **POST** `/orders`
- **Body**:
```json
{
  "customer_name": "John Doe",
  "items": ["Item 1", "Item 2"],
  "total_amount": 99.99
}
```
- **Response**: 201 Created
```json
{
  "id": "uuid-here",
  "customer_name": "John Doe",
  "items": ["Item 1", "Item 2"],
  "total_amount": 99.99,
  "status": "pending",
  "created_at": "2025-01-01T12:00:00Z",
  "updated_at": "2025-01-01T12:00:00Z"
}
```

### 2. Get Order by ID
- **GET** `/orders/{id}`
- **Response**: 200 OK (order object) or 404 Not Found

### 3. List All Orders
- **GET** `/orders`
- **Response**: 200 OK
```json
[
  {
    "id": "uuid-here",
    "customer_name": "John Doe",
    ...
  }
]
```

### 4. Update Order Status
- **PATCH** `/orders/{id}/status`
- **Body**:
```json
{
  "status": "shipped"
}
```
- **Valid statuses**: `pending`, `processing`, `shipped`, `delivered`, `cancelled`
- **Response**: 200 OK (updated order) or 404 Not Found

### 5. Delete Order
- **DELETE** `/orders/{id}`
- **Response**: 204 No Content or 404 Not Found

## Example Usage with curl

```bash
# Create an order
curl -X POST http://127.0.0.1:8080/orders \
  -H "Content-Type: application/json" \
  -d '{"customer_name":"John Doe","items":["Widget","Gadget"],"total_amount":150.50}'

# List all orders
curl http://127.0.0.1:8080/orders

# Get specific order (replace {id} with actual UUID)
curl http://127.0.0.1:8080/orders/{id}

# Update order status
curl -X PATCH http://127.0.0.1:8080/orders/{id}/status \
  -H "Content-Type: application/json" \
  -d '{"status":"shipped"}'

# Delete order
curl -X DELETE http://127.0.0.1:8080/orders/{id}
```

## Error Handling

The API implements comprehensive error handling:

### Validation Errors (400 Bad Request)
- Empty customer name
- Empty items list
- Total amount ≤ 0

### Not Found Errors (404)
- Order ID doesn't exist

### Response Format
```json
{
  "error": "Descriptive error message"
}
```

## Design Decisions

### Framework Choice
**Actix-web** was chosen for its:
- High performance and async support
- Mature ecosystem
- Excellent documentation
- Built-in testing utilities

### Data Storage
**In-memory HashMap** with Mutex for:
- Simplicity for a coding exercise
- Thread-safe concurrent access
- Fast operations without database overhead
- Easy to replace with persistent storage if needed

### Order Status Enum
Predefined states prevent invalid status values and provide type safety.

### UUID for IDs
UUIDs provide:
- Globally unique identifiers
- No sequential ID guessing
- Easy distributed system support

### Timestamp Tracking
Both `created_at` and `updated_at` fields enable:
- Audit trails
- Sorting by creation/modification
- Future analytics capabilities

### Error Response Structure
Consistent JSON error format makes client error handling predictable.

## Trade-offs

1. **In-memory storage**: Fast but data is lost on restart. For production, use PostgreSQL/SQLite.
2. **No authentication**: Would need JWT/OAuth for production.
3. **No pagination**: List endpoint returns all orders. Should add pagination for large datasets.
4. **Mutex locking**: Works for this scale but could be a bottleneck.
