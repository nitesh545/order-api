use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: Uuid,
    customer_name: String,
    items: Vec<String>,
    total_amount: f64,
    status: OrderStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CreateOrderRequest {
    customer_name: String,
    items: Vec<String>,
    total_amount: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct UpdateStatusRequest {
    status: OrderStatus,
}

#[allow(dead_code)]
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[allow(dead_code)]
struct AppState {
    pub orders: Mutex<HashMap<Uuid, Order>>,
}

// Create new order
async fn create_order(
    data: web::Data<AppState>,
    req: web::Json<CreateOrderRequest>,
) -> impl Responder {
    if req.customer_name.trim().is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Customer name cannot be empty".to_string(),
        });
    }

    if req.items.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Order must contain at least one item".to_string(),
        });
    }

    if req.total_amount <= 0.0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Total amount must be greater than zero".to_string(),
        });
    }

    let order = Order {
        id: Uuid::new_v4(),
        customer_name: req.customer_name.clone(),
        items: req.items.clone(),
        total_amount: req.total_amount,
        status: OrderStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let mut orders = data.orders.lock().unwrap();
    orders.insert(order.id, order.clone());

    println!("New order created successfully");
    HttpResponse::Created().json(order)
}

// Get order by ID
async fn get_order(data: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let order_id = path.into_inner();
    let orders = data.orders.lock().unwrap();

    match orders.get(&order_id) {
        Some(order) => HttpResponse::Ok().json(order),
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Order with ID {order_id} not found"),
        }),
    }
}

// List all orders
async fn list_orders(data: web::Data<AppState>) -> impl Responder {
    let orders = data.orders.lock().unwrap();
    let order_list: Vec<Order> = orders.values().cloned().collect();
    println!("listing orders");
    HttpResponse::Ok().json(order_list)
}

// Update order status
async fn update_order_status(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateStatusRequest>,
) -> impl Responder {
    let order_id = path.into_inner();
    let mut orders = data.orders.lock().unwrap();

    match orders.get_mut(&order_id) {
        Some(order) => {
            order.status = req.status.clone();
            order.updated_at = Utc::now();
            println!("updated successfully. Id: {order_id}");
            HttpResponse::Ok().json(order.clone())
        }
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Order with ID {order_id} not found"),
        }),
    }
}

// Delete order
async fn delete_order(data: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let order_id = path.into_inner();
    let mut orders = data.orders.lock().unwrap();

    match orders.remove(&order_id) {
        Some(_) => {
            println!("Order deleted. ID: {order_id}");
            HttpResponse::NoContent().finish()
        }
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Order with ID {order_id} not found"),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        orders: Mutex::new(HashMap::new()),
    });

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/create-order", web::post().to(create_order))
            .route("/list-orders", web::get().to(list_orders))
            .route("/orders/{id}", web::get().to(get_order))
            .route("/orders/{id}/status", web::patch().to(update_order_status))
            .route("/orders/{id}", web::delete().to(delete_order))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
