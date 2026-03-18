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
    // El JSON exacto solicitado en la rúbrica para la API 2
    Json(json!({
        "Instancia": "Instancia #2 - API #2",
        "Curso": "Seminario de Sistemas 1",
        "Estudiante": "Estudiante - 202302220",
        "Lenguaje": "Rust",
    }))
}