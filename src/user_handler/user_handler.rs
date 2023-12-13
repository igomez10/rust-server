use super::user_handler_trait::UserHandlerTrait;
use crate::{models::models::User, user_controller::user_controller_trait::UserControllerTrait};

use std::sync::{Arc, Mutex};

// UserHandler is a struct that implements UserHandlerTrait
pub struct UserHandler {
    user_controller: Arc<Mutex<dyn UserControllerTrait>>,
}
impl UserHandler {
    pub fn new(controller: impl UserControllerTrait + 'static) -> UserHandler {
        UserHandler {
            user_controller: Arc::new(Mutex::new(controller)),
        }
    }
}
impl UserHandlerTrait for UserHandler {
    fn add_user(&self, user: User) {
        self.user_controller.lock().unwrap().add_user(user);
    }

    fn get_user(&self, id: i32) -> Option<User> {
        return self.user_controller.lock().unwrap().get_user(id);
    }

    fn list_users(&self) -> Vec<User> {
        return self.user_controller.lock().unwrap().list_users();
    }

    fn remove_user(&self, id: i32) {
        self.user_controller.lock().unwrap().remove_user(id);
    }
}
