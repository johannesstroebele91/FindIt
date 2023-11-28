#[derive(Serialize, Deserialize, Clone)]
struct Apartment {
    id: u32,
    name: String,
    rooms: Vec<Room>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Room {
    id: u32,
    name: String,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct AppState {
    apartments: Vec<Apartment>,
}

use axum::{response::Html, routing::get, Router, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

const ADDR: &'static str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    let apartments_string = std::fs::read_to_string("apartment_mock.json").unwrap();
    let app_state: AppState = serde_json::from_str(&apartments_string).unwrap();
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/apartment", get(get_apartments))
        .with_state(app_state);

    // run it
    let listener = tokio::net::TcpListener::bind(ADDR)
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_apartments(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.apartments)
}

async fn handler() -> Html<&'static str> {
    Html("<a>Hello, World!</a>")
}