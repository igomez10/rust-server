use crate::models::models::User;
use log::{debug, error, info, warn};
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

#[get("/exec")]
async fn handler_exec() -> String {
    // call makeHTTPRequest
    let res = make_http_request().await;
    return res;
}

#[get("/users")]
fn list_users(state: &State<AppState>) -> String {
    let lock_res = state.user_handler.lock();
    match lock_res {
        Ok(handler) => {
            let users = handler.list_users();
            let res = serde_json::to_string(&users).unwrap();
            return res.to_string();
        }
        Err(e) => {
            log::error!("Error: {}", e);
            return "Error".to_string();
        }
    }
}

#[get("/users/<id>")]
fn get_user(id: i32, state: &State<AppState>) -> String {
    // get name
    let handler = match state.user_handler.lock() {
        Ok(handler) => handler,
        Err(e) => {
            log::error!("Error: {}", e);
            return "Error".to_string();
        }
    };

    let user = match handler.get_user(id) {
        Ok(user) => user,
        Err(e) => {
            log::error!("Error: {}", e);
            return "Error".to_string();
        }
    };

    let res = match serde_json::to_string(&user) {
        Ok(res) => res,
        Err(e) => {
            log::error!("Error formatting json: {}", e);
            return "Error".to_string();
        }
    };

    return res.to_string();
}

// post json to /users to create user
#[post("/users", data = "<user>")]
fn create_user(user: Json<User>, state: &State<AppState>) {
    let handler = match state.user_handler.lock() {
        Ok(handler) => handler,
        Err(e) => {
            log::error!("Error: {}", e);
            return;
        }
    };
    let user_obj = user.into_inner();
    handler.add_user(user_obj);
}

#[delete("/users/<id>")]
fn delete_user(id: i32, state: &State<AppState>) -> String {
    // get name
    let handler = match state.user_handler.lock() {
        Ok(handler) => handler,
        Err(e) => {
            log::error!("Error: {}", e);
            return "Error".to_string();
        }
    };

    // fetch user and delete it
    let user_to_remove = match handler.get_user(id) {
        Ok(user) => user,
        Err(e) => {
            log::error!("Error: {}", e);
            return "Error".to_string();
        }
    };

    handler.remove_user(id);
    let res_json = match serde_json::to_string(&user_to_remove) {
        Ok(res) => res,
        Err(e) => {
            log::error!("Error formatting json: {}", e);
            return "Error".to_string();
        }
    };

    return res_json.to_string();
}

async fn make_http_request() -> String {
    let client = reqwest::Client::new();
    let res = client
        .get("https://www.rust-lang.org")
        .header("User-Agent", "warp")
        .send()
        .await
        .unwrap();

    log::info!("Status: {}", res.status());
    return res.text().await.unwrap();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::init();
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
