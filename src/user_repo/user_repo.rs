use super::user_repo_trait::UserRepoTrait;
use crate::models::models::User;

// User repo stores users in a hashmap
pub struct UserRepo {
    users: Vec<User>,
}

impl UserRepo {
    pub fn new() -> UserRepo {
        let initial_users = vec![
            User::new(1, "Alice".to_string(), "30".to_string(), "".to_string()),
            User::new(1, "Bob".to_string(), "29".to_string(), "".to_string()),
        ];

        UserRepo {
            users: initial_users,
        }
    }
}

impl UserRepoTrait for UserRepo {
    fn create_user(&mut self, user: User) {
        self.users.push(user);
    }
    fn get_user(&mut self, id: i32) -> Result<User, Box<dyn std::error::Error>> {
        for user in &self.users {
            if user.id == id {
                return Ok(user.clone());
            }
        }
        return Err("User not found".into());
    }

    fn list_users(&mut self) -> Vec<User> {
        let users = self.users.clone();
        return users;
    }

    fn remove_user(&mut self, id: i32) {
        self.users.retain(|user| user.id != id);
    }

    fn find_user(&mut self, _name: String) -> Option<User> {
        todo!()
    }
}

impl Default for UserRepo {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UserSearch {
    name: String,
    age: i32,
}
