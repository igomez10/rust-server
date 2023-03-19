use std::sync::{Arc, Mutex};

use crate::{
    models,
    user_repo::{UserRepo, UserRepoTrait},
};

// trait
pub trait UserHandlerTrait {
    fn get_name(&self) -> String;
    fn set_name(&self, name: String);
    fn add_user(&self, user: models::User);
    fn get_user(&self, id: i32) -> Option<models::User>;
    fn list_users(&self) -> Vec<models::User>;
    fn remove_user(&self, id: i32);
}

pub struct UserHandler {
    user_repo: Arc<Mutex<UserRepo>>,
}

impl UserHandler {
    pub fn new(user_repo: UserRepo) -> UserHandler {
        UserHandler {
            user_repo: Arc::new(Mutex::new(user_repo)),
        }
    }

    pub fn get_name(&self) -> String {
        return self.user_repo.lock().unwrap().get_name();
    }
    pub fn set_name(&self, name: String) {
        self.user_repo.lock().unwrap().set_name(name);
    }
    pub fn add_user(&self, user: models::User) {
        self.user_repo.lock().unwrap().add_user(user);
    }
    pub fn get_user(&self, id: i32) -> Option<models::User> {
        self.user_repo.lock().unwrap().get_user(id)
    }
    pub fn list_users(&self) -> Vec<models::User> {
        self.user_repo.lock().unwrap().list_users()
    }
    pub fn remove_user(&self, id: i32) {
        self.user_repo.lock().unwrap().remove_user(id);
    }
}
