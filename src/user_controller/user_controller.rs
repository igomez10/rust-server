use super::user_controller_trait::UserControllerTrait;
use crate::{models::models::User, user_repo::user_repo_trait::UserRepoTrait};

pub struct UserController {
    user_repo: Box<dyn UserRepoTrait>,
}

impl UserController {
    pub fn new(user_repo: Box<dyn UserRepoTrait>) -> UserController {
        UserController { user_repo }
    }
}

impl UserControllerTrait for UserController {
    fn add_user(&mut self, user: User) {
        self.user_repo.create_user(user);
    }
    fn get_user(&mut self, id: i32) -> Result<User, Box<dyn std::error::Error>> {
        match self.user_repo.get_user(id) {
            Ok(user) => Ok(user),
            Err(err) => Err(err),
        }
    }
    fn list_users(&mut self) -> Vec<User> {
        self.user_repo.list_users()
    }
    fn remove_user(&mut self, id: i32) {
        self.user_repo.remove_user(id);
    }
}
