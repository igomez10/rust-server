use crate::user_controller::UserCtrlTrait;
use std::sync::{Arc, Mutex};

use crate::models;

// trait
pub trait UserHandlerTrait: Send + Sync {
    fn get_name(&self) -> String;
    fn set_name(&self, name: String);
    fn add_user(&self, user: models::User);
    fn get_user(&self, id: i32) -> Option<models::User>;
    fn list_users(&self) -> Vec<models::User>;
    fn remove_user(&self, id: i32);
}

pub struct UserHandler {
    user_controller: Arc<Mutex<dyn UserCtrlTrait>>,
}

impl UserHandler {
    pub fn new(user_repo: impl UserCtrlTrait + 'static) -> UserHandler {
        UserHandler {
            user_controller: Arc::new(Mutex::new(user_repo)),
        }
    }
}

impl UserHandlerTrait for UserHandler {
    fn get_name(&self) -> String {
        return self.user_controller.lock().unwrap().get_name();
    }

    fn set_name(&self, name: String) {
        self.user_controller.lock().unwrap().set_name(name);
    }

    fn add_user(&self, user: models::User) {
        self.user_controller.lock().unwrap().add_user(user);
    }

    fn get_user(&self, id: i32) -> Option<models::User> {
        return self.user_controller.lock().unwrap().get_user(id);
    }

    fn list_users(&self) -> Vec<models::User> {
        return self.user_controller.lock().unwrap().list_users();
    }

    fn remove_user(&self, id: i32) {
        self.user_controller.lock().unwrap().remove_user(id);
    }
}
