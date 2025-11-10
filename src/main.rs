use actix_web::{App, HttpServer, web};
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
struct AppState {
    pub orders: Mutex<HashMap<Uuid, Order>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        orders: Mutex::new(HashMap::new()),
    });

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || App::new().app_data(app_state.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
