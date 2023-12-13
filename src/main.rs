use crate::models::models::User;
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};
use user_handler::user_handler::UserHandler;
use user_handler::user_handler_trait::UserHandlerTrait;
mod middlewares;
mod models;
mod square;
mod user_controller;
mod user_handler;
mod user_repo;

#[macro_use]
extern crate rocket;

pub struct AppState {
    pub user_handler: Arc<Mutex<dyn UserHandlerTrait>>,
}

impl AppState {
    pub fn new(user_handler: impl UserHandlerTrait + 'static) -> AppState {
        let mutex = Mutex::new(user_handler);
        let arc = Arc::new(mutex);

        AppState { user_handler: arc }
    }
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

// example of async function
#[get("/exec")]
async fn handler_exec() -> String {
    // call makeHTTPRequest
    let res = make_http_request().await;
    return res;
}

#[get("/users")]
fn list_users(state: &State<AppState>) -> String {
    // get name
    let users = state.user_handler.lock().unwrap().list_users();
    let res = serde_json::to_string(&users).unwrap();

    return res.to_string();
}

#[get("/users/<id>")]
fn get_user(id: i32, state: &State<AppState>) -> String {
    // get name
    let user = state.user_handler.lock().unwrap().get_user(id);
    let res = serde_json::to_string(&user).unwrap();
    return res;
}

// post json to /users to create user
#[post("/users", data = "<user>")]
fn create_user(user: Json<User>, state: &State<AppState>) {
    // get name
    state
        .user_handler
        .lock()
        .unwrap()
        .add_user(user.into_inner());
}

#[delete("/users/<id>")]
fn delete_user(id: i32, state: &State<AppState>) {
    // get name
    state.user_handler.lock().unwrap().remove_user(id);
}

async fn make_http_request() -> String {
    let client = reqwest::Client::new();
    let res = client
        .get("https://www.rust-lang.org")
        .header("User-Agent", "warp")
        .send()
        .await
        .unwrap();

    println!("Status: {}", res.status());
    return res.text().await.unwrap();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let user_repo = user_repo::user_repo::UserRepo::new();
    let user_controller =
        user_controller::user_controller::UserController::new(Box::new(user_repo));
    let user_handler = UserHandler::new(user_controller);
    let app_state = AppState::new(user_handler);

    let counter_middleware = middlewares::counter::Counter {
        get: AtomicUsize::new(0),
        post: AtomicUsize::new(0),
    };

    let request_id_middleware = middlewares::request_id::RequestId {};

    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                hello,
                handler_exec,
                list_users,
                get_user,
                create_user,
                delete_user
            ],
        )
        .attach(counter_middleware)
        .attach(request_id_middleware)
        .manage(app_state)
        .launch()
        .await?;

    Ok(())
}
