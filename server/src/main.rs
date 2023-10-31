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
        .route("/", get(get_todos))
}


#[tokio::main]
async fn main() {
    let app = create_app();
    Server::bind(&"0.0.0.0:3003".parse().unwrap())
        .serve(app.into_make_service()).await.unwrap();
}
