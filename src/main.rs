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
    pub todo_db: Arc<Mutex<Vec<i32>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            todo_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
struct UserHandler {
    user_repo: userrepo::UserRepo,
    count: Arc<Mutex<i32>>,
}

impl UserHandler {
    fn new(user_repo: userrepo::UserRepo) -> UserHandler {
        UserHandler {
            user_repo: user_repo,
            count: Arc::new(Mutex::new(0)),
        }
    }
    fn set_name(&mut self, name: String) {
        self.user_repo.set_name(name);
    }
    fn get_name(&self) -> String {
        self.user_repo.get_name()
    }
    fn increase_count(&mut self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/exec")]
async fn handlerexec() -> String {
    // call makehttprequest
    let res = makehttprequest().await;
    return res;
}

#[get("/count")]
fn getcount(uh: &State<UserHandler>) -> String {
    // increase count by 1
    let mut count = uh.count.lock().unwrap();
    *count += 1;

    return format!("Count: {}", count);
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

    rocket::build()
        .mount("/", routes![hello, handlerexec, getcount])
        .manage(user_handler)
}
