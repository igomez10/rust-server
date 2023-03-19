use std::sync::{Arc, Mutex};

use crate::{models, user_repo::UserRepoTrait};

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
    user_repo: Arc<Mutex<dyn UserRepoTrait>>,
}

impl UserHandler {
    pub fn new(user_repo: impl UserRepoTrait + 'static) -> UserHandler {
        UserHandler {
            user_repo: Arc::new(Mutex::new(user_repo)),
        }
    }
}

impl UserHandlerTrait for UserHandler {
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
        return self.user_repo.lock().unwrap().get_user(id);
    }

    fn list_users(&self) -> Vec<models::User> {
        return self.user_repo.lock().unwrap().list_users();
    }

    fn remove_user(&self, id: i32) {
        self.user_repo.lock().unwrap().remove_user(id);
    }
}
