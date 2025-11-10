use actix_web::{App, HttpServer, web};
use std::collections::HashMap;
use std::sync::Mutex;
mod endpoints;
use endpoints::{
    AppState, create_order, delete_order, get_order, list_orders, update_order_status,
};

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
