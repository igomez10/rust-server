// import user from models.rs User
use crate::models::User;

// User repo stores users in a hashmap
pub struct UserRepo {
    users: Vec<User>,
    name: String,
}

impl UserRepo {
    pub fn new() -> UserRepo {
        // UserRepo { users: Vec::new() }
        UserRepo {
            name: "UserRepo".to_string(),
            users: Vec::new(),
        }
    }
}

impl UserRepoTrait for UserRepo {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn get_name(&mut self) -> String {
        self.name.clone()
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    fn get_user(&mut self, id: i32) -> Option<User> {
        self.users.iter().find(|user| user.id == id).cloned()
    }

    fn list_users(&mut self) -> Vec<User> {
        // create copy of users
        let users = self.users.clone();
        // sort users by id
        return users;
    }

    fn remove_user(&mut self, id: i32) {
        self.users.retain(|user| user.id != id);
    }
}

impl Default for UserRepo {
    fn default() -> Self {
        Self::new()
    }
}

// trait
pub trait UserRepoTrait: Send + Sync {
    fn get_name(&mut self) -> String;
    fn set_name(&mut self, name: String);
    fn add_user(&mut self, user: User);
    fn get_user(&mut self, id: i32) -> Option<User>;
    fn list_users(&mut self) -> Vec<User>;
    fn remove_user(&mut self, id: i32);
}

// mock
pub struct MockUserRepo {
    //mutable name field
    pub name: String,
    pub count: i32,

    // user to return
    pub user_to_return: Option<User>,
    pub name_to_return: Option<String>,
    pub users_to_return: Option<Vec<User>>,
}

impl UserRepoTrait for MockUserRepo {
    fn get_name(&mut self) -> String {
        self.count += 1;
        self.name.clone()
    }
    fn set_name(&mut self, name: String) {
        self.count += 1;
        self.name = name;
    }
    fn add_user(&mut self, _: User) {
        // do nothing
        self.count += 1;
    }
    fn get_user(&mut self, _: i32) -> Option<User> {
        self.count += 1;
        self.user_to_return.clone()
    }
    fn list_users(&mut self) -> Vec<User> {
        self.count += 1;
        Vec::new()
    }
    fn remove_user(&mut self, _: i32) {}
}

// tests
#[cfg(test)]
mod tests {
    use super::UserRepoTrait;

    #[test]
    fn test_add_user() {
        let mut user_repo = super::UserRepo::new();
        let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.add_user(user);
        assert_eq!(user_repo.users.len(), 1);
    }
    #[test]
    fn test_get_user() {
        let mut user_repo = super::UserRepo::new();
        let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.add_user(user);
        let user = user_repo.get_user(1);
        assert_eq!(user.unwrap().name, "John");
    }
    #[test]
    fn test_list_users() {
        let mut user_repo = super::UserRepo::new();
        let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.add_user(user);
        let user = super::User::new(2, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.add_user(user);
        let user = super::User::new(3, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.add_user(user);

        let users = user_repo.list_users();
        assert_eq!(users.len(), 3);
    }

    #[test]
    fn test_remove_user() {
        let mut user_repo = super::UserRepo::new();
        let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.add_user(user);
        let user = super::User::new(2, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.add_user(user);
        let user = super::User::new(3, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.add_user(user);

        user_repo.remove_user(2);
        let users = user_repo.list_users();
        assert_eq!(users.len(), 2);
    }

    #[test]
    fn test_get_name() {
        let mut user_repo = super::UserRepo::new();
        let name = user_repo.get_name();
        assert_eq!(name, "UserRepo");
    }
}
