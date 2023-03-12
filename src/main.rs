use rocket::{serde::json::Json, State};
use std::sync::{Arc, Mutex};
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
    fn add_user(&self, user: models::User) {
        self.user_repo.lock().unwrap().add_user(user);
    }
    fn get_user(&self, id: i32) -> Option<models::User> {
        self.user_repo.lock().unwrap().get_user(id).cloned()
    }
    fn list_users(&self) -> Vec<models::User> {
        self.user_repo.lock().unwrap().list_users()
    }
    fn remove_user(&self, id: i32) {
        self.user_repo.lock().unwrap().remove_user(id);
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
        .set_name(name.to_string());
}

#[get("/name")]
fn getname(state: &State<AppState>) -> String {
    // get name
    return state.user_handler.lock().unwrap().get_name();
}

#[get("/users")]
fn listusers(state: &State<AppState>) -> String {
    // get name
    let users = state.user_handler.lock().unwrap().list_users();
    let mut res = String::new();
    for user in users {
        res.push_str(&format!("{} ", user.name));
    }
    return res;
}

#[get("/users/<id>")]
fn getuser(id: i32, state: &State<AppState>) -> String {
    // get name
    let user = state.user_handler.lock().unwrap().get_user(id);
    match user {
        Some(user) => return user.name,
        None => return "User not found".to_string(),
    }
}

// post json to /users to create user
#[post("/users", data = "<user>")]
fn createuser(user: Json<models::User>, state: &State<AppState>) {
    // get name
    state
        .user_handler
        .lock()
        .unwrap()
        .add_user(user.into_inner());
}

#[delete("/users/<id>")]
fn deleteuser(id: i32, state: &State<AppState>) {
    // get name
    state.user_handler.lock().unwrap().remove_user(id);
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
        .mount(
            "/",
            routes![
                hello,
                handlerexec,
                postname,
                getname,
                listusers,
                getuser,
                createuser,
                deleteuser
            ],
        )
        .manage(app_state)
}
