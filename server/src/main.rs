use axum::http::StatusCode;
use axum::{response::Html, routing::get, Router, Json};
use axum::extract::{State, FromRef};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

const ADDR: &'static str = "127.0.0.1:3000";


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

#[derive(Serialize, Deserialize, sqlx::FromRow, Default, Clone)]
struct Item {
    id: u32,
    name: String,
    description: String,
}


#[derive(Clone)]
struct AppState {
    apartment_state: ApartmentState,
    pool_state: SqlitePoolState,    
}

#[derive(Serialize, Deserialize, Clone)]
struct ApartmentState {
    apartments: Vec<Apartment>,
}

impl FromRef<AppState> for ApartmentState {
    fn from_ref(app_state: &AppState) -> ApartmentState {
        app_state.apartment_state.clone()
    }

}

#[derive(Clone)]
struct SqlitePoolState {
    pool: SqlitePool

}

impl FromRef<AppState> for SqlitePoolState {
    fn from_ref(app_state: &AppState) -> SqlitePoolState {
        app_state.pool_state.clone()
    }
}


#[tokio::main]
async fn main() {
    
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();


    {
        let mut conn = pool.acquire().await.unwrap();

//         let id = sqlx::query!(
//             r#"
// INSERT INTO items ( name, description)
// VALUES ( ?1 ?2 )
//             "
//         )
//     }

        sqlx::query("CREATE TABLE items ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, description TEXT NOT NULL);")
            .execute(&mut *conn)
            .await
            .unwrap();

        let id = sqlx::query("INSERT INTO items ( name, description ) VALUES ('Pan', 'For frying');")
            .execute(&mut *conn)
            .await
            .unwrap()
            .last_insert_rowid();
        println!("Created id: {id}");
    }
    
    
    
    // let apartments_string = std::fs::read_to_string("apartment_mock.json").unwrap();
    let apartments_string = include_str!("../apartment_mock.json");
    let apartment_state: ApartmentState = serde_json::from_str(&apartments_string).unwrap();
    
    let app_state = AppState {
        apartment_state,
        pool_state: SqlitePoolState { pool },
    };

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/apartment", get(get_apartments))
        .route("/item", get(get_items))
        .with_state(app_state);

    // run it
    let listener = tokio::net::TcpListener::bind(ADDR)
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


async fn get_apartments(State(state): State<ApartmentState>) -> impl IntoResponse {
    Json(state.apartments)
}

async fn get_items(State(pool_state): State<SqlitePoolState>) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM items")
        .fetch_one(&pool_state.pool)
        .await {
            Ok(item) => {
                Json(item)
            },
            Err(e) => {
                println!("Error when running select {}", e);
                Json(Item::default())
            },
        }
}

async fn handler() -> Html<&'static str> {
    Html("<a>Hello, World!</a>")
}