use axum::{routing::get, Router, response::Json};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Configuramos CORS para permitir peticiones desde cualquier origen
    let app = Router::new()
        .route("/get-data", get(get_data))
        .layer(CorsLayer::permissive());

    println!("API en Rust escuchando en el puerto 8080...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_data() -> Json<Value> {
    // Un JSON ligeramente distinto para identificar esta API
    Json(json!({
        "api": "Rust",
        "mensaje": "Respuesta exitosa desde la API en Rust",
        "estado": "Activo"
    }))
}