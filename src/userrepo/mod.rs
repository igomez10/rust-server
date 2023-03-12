// import user from models.rs User
use crate::models::User;
use std::collections::HashMap;

// User repo stores users in a hashmap
pub struct UserRepo {
    // users: Vec<User>,
    name: String,
}

impl UserRepo {
    pub fn new() -> UserRepo {
        // UserRepo { users: Vec::new() }
        UserRepo {
            name: "UserRepo".to_string(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Default for UserRepo {
    fn default() -> Self {
        Self::new()
    }
}
impl Clone for UserRepo {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    // #[test]
    // fn test_add_user() {
    //     let mut user_repo = super::UserRepo::new();
    //     let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
    //     user_repo.add_user(user);
    //     assert_eq!(user_repo.users.len(), 1);
    // }
    // #[test]
    // fn test_get_user() {
    //     let mut user_repo = super::UserRepo::new();
    //     let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
    //     user_repo.add_user(user);
    //     let user = user_repo.get_user(1);
    //     assert_eq!(user.unwrap().name, "John");
    // }
    // #[test]
    // fn test_list_users() {
    //     let mut user_repo = super::UserRepo::new();
    //     let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
    //     user_repo.add_user(user);
    //     let user = super::User::new(2, "John".to_string(), "hey".to_string(), "123".to_string());
    //     user_repo.add_user(user);
    //     let user = super::User::new(3, "John".to_string(), "hey".to_string(), "123".to_string());
    //     user_repo.add_user(user);

    //     let users = user_repo.list_users();
    //     assert_eq!(users.len(), 3);
    // }
}
