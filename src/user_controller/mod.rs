use crate::models::User;
use crate::user_repo::UserRepoTrait;

pub trait UserCtrlTrait: Sync + Send {
    fn get_name(&mut self) -> String;
    fn set_name(&mut self, name: String);
    fn add_user(&mut self, user: User);
    fn get_user(&mut self, id: i32) -> Option<User>;
    fn list_users(&mut self) -> Vec<User>;
    fn remove_user(&mut self, id: i32);
}

pub struct UserCtrl {
    user_repo: Box<dyn UserRepoTrait>,
}

impl UserCtrl {
    pub fn new(user_repo: Box<dyn UserRepoTrait>) -> UserCtrl {
        UserCtrl { user_repo }
    }
}

impl UserCtrlTrait for UserCtrl {
    fn get_name(&mut self) -> String {
        self.user_repo.get_name()
    }
    fn set_name(&mut self, name: String) {
        self.user_repo.set_name(name);
    }
    fn add_user(&mut self, user: User) {
        self.user_repo.add_user(user);
    }
    fn get_user(&mut self, id: i32) -> Option<User> {
        self.user_repo.get_user(id)
    }
    fn list_users(&mut self) -> Vec<User> {
        self.user_repo.list_users()
    }
    fn remove_user(&mut self, id: i32) {
        self.user_repo.remove_user(id);
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::UserRepoTrait;
    use crate::{models::User, user_repo::MockUserRepo};
    #[test]
    fn test_add_user() {
        let mut mock_user_repo = MockUserRepo {
            name: "John".to_string(),
            count: 0,
            user_to_return: None,
            name_to_return: None,
            users_to_return: None,
        };

        let user = User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
        mock_user_repo.add_user(user);
        assert_eq!(mock_user_repo.count, 1);
    }

    #[test]
    fn test_get_user() {
        let expected_user = User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());

        let mut mock_user_repo = MockUserRepo {
            name: "John".to_string(),
            count: 0,
            user_to_return: Some(expected_user.clone()),
            name_to_return: None,
            users_to_return: None,
        };

        let actual_user = mock_user_repo.get_user(1);
        assert_eq!(actual_user.clone().unwrap().name, expected_user.name);
        assert_eq!(actual_user.clone().unwrap().id, expected_user.id);
        assert_eq!(mock_user_repo.count, 1);
    }
}
