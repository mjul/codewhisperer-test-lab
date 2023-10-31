// Using Tokio and Axum, create a TODO list application
// exposed via REST JSON data structures.
// The back-end uses SQLite for persistence.
// Since it is a small app, we keep everything in this one file

use axum::{routing::get, Router, Server, Json, http::StatusCode};
use serde::Serialize;


#[derive(Serialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Serialize)]
struct TodoList {
    todos: Vec<Todo>,
}


async fn get_todos() -> Result<Json<TodoList>, StatusCode> {
    // Get the TODOs from SQLite
    // Return them as a JSON object
    
    
    let todos = vec![
        Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Learn GraphQL".to_string(),
            completed: false,
        },
    ];
    let result = TodoList { todos };
    Ok(Json(result))
}


fn create_app() -> Router {
    Router::new()
        .route("/api/todos", get(get_todos))
}


#[tokio::main]
async fn main() {
    let app = create_app();

    // Initialize tracing
    let default_collector = tracing_subscriber::fmt()
        .with_env_filter("info,tower_http=debug,server=debug")
        // build but do not install the subscriber.
        .finish();
    tracing::subscriber::set_global_default(default_collector)
        .expect("setting default subscriber failed");

    // add and turn on logging
    let app = app.layer(tower_http::trace::TraceLayer::new_for_http());
    //tracing_subscriber::fmt::init();

    // start the server
    let addr = "0.0.0.0:3003".parse().unwrap();
    tracing::info!("Starting server on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service()).await.unwrap();
}
