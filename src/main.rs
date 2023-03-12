use std::{
    ops::Add,
    sync::{Arc, Mutex},
};

use rocket::{request::FromRequest, State};

mod math;
mod models;
mod square;
mod userrepo;

#[macro_use]
extern crate rocket;

pub struct AppState {
    pub user_handler: Arc<Mutex<UserHandler>>,
}

impl AppState {
    pub fn new(user_handler: UserHandler) -> AppState {
        AppState {
            user_handler: Arc::new(Mutex::new(user_handler)),
        }
    }
}
pub struct UserHandler {
    user_repo: Arc<Mutex<userrepo::UserRepo>>,
}

impl UserHandler {
    fn new(user_repo: userrepo::UserRepo) -> UserHandler {
        UserHandler {
            user_repo: Arc::new(Mutex::new(user_repo)),
        }
    }

    fn get_name(&self) -> String {
        return self.user_repo.lock().unwrap().get_name();
    }
    fn set_name(&self, name: String) {
        self.user_repo.lock().unwrap().set_name(name);
    }
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

// example of async function
#[get("/exec")]
async fn handlerexec() -> String {
    // call makehttprequest
    let res = makehttprequest().await;
    return res;
}

#[post("/name/<name>")]
fn postname(name: &str, state: &State<AppState>) {
    // get name
    state
        .user_handler
        .lock()
        .unwrap()
        .user_repo
        .lock()
        .unwrap()
        .set_name(name.to_string());
}

#[get("/name")]
fn getname(state: &State<AppState>) -> String {
    // get name
    return state
        .user_handler
        .lock()
        .unwrap()
        .user_repo
        .lock()
        .unwrap()
        .get_name();
}

async fn makehttprequest() -> String {
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

#[launch]
fn rocket() -> _ {
    let user_repo = userrepo::UserRepo::new();
    let user_handler = UserHandler::new(user_repo);
    let app_state = AppState::new(user_handler);

    rocket::build()
        .mount("/", routes![hello, handlerexec, postname, getname])
        .manage(app_state)
}
