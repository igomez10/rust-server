// tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_add_user() {
        let mut user_repo = super::UserRepo::new();
        let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.create_user(user);
        assert_eq!(user_repo.users.len(), 1);
    }
    #[test]
    fn test_get_user() {
        let mut user_repo = super::UserRepo::new();
        let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.create_user(user);
        let user = user_repo.get_user(1);
        assert_eq!(user.unwrap().name, "John");
    }
    #[test]
    fn test_list_users() {
        let mut user_repo = super::UserRepo::new();
        let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.create_user(user);
        let user = super::User::new(2, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.create_user(user);
        let user = super::User::new(3, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.create_user(user);

        let users = user_repo.list_users();
        assert_eq!(users.len(), 3);
    }

    #[test]
    fn test_remove_user() {
        let mut user_repo = super::UserRepo::new();
        let user = super::User::new(1, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.create_user(user);
        let user = super::User::new(2, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.create_user(user);
        let user = super::User::new(3, "John".to_string(), "hey".to_string(), "123".to_string());
        user_repo.create_user(user);

        user_repo.remove_user(2);
        let users = user_repo.list_users();
        assert_eq!(users.len(), 2);
    }
}
