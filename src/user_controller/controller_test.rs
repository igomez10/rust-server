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
        mock_user_repo.create_user(user);
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
