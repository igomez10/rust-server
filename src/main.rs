use handlers::user::UserHandler;
use rocket::{serde::json::Json, State};
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

mod math;
mod models;
mod square;
mod user_controller;
mod user_repo;
mod middlewares {
    pub mod counter;
    pub mod request_id;
}

mod handlers {
    pub mod user;
}

#[macro_use]
extern crate rocket;

pub struct AppState {
    pub user_handler: Arc<Mutex<UserHandler>>,
}

impl AppState {
    pub fn new(user_handler: UserHandler) -> AppState {
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

#[post("/name/<name>")]
fn post_name(name: &str, state: &State<AppState>) {
    // get name
    state
        .user_handler
        .lock()
        .unwrap()
        .set_name(name.to_string());
}

#[get("/name")]
fn get_name(state: &State<AppState>) -> String {
    // get name
    return state.user_handler.lock().unwrap().get_name();
}

#[get("/users")]
fn list_users(state: &State<AppState>) -> String {
    // get name
    let users = state.user_handler.lock().unwrap().list_users();
    let mut res = String::new();
    for user in users {
        res.push_str(&format!("{} ", user.name));
    }
    return res;
}

#[get("/users/<id>")]
fn get_user(id: i32, state: &State<AppState>) -> String {
    // get name
    let user = state.user_handler.lock().unwrap().get_user(id);
    return match user {
        Some(user) => user.name,
        None => "User not found".to_string(),
    };
}

// post json to /users to create user
#[post("/users", data = "<user>")]
fn create_user(user: Json<models::User>, state: &State<AppState>) {
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
    let user_repo = user_repo::UserRepo::new();
    let user_handler = UserHandler::new(user_repo);
    let app_state = AppState::new(user_handler);
    // mount in port 8080

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
                post_name,
                get_name,
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

// fn main() {}
