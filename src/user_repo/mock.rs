mod mock {
    use crate::{models::User, user_repo::user_repo_trait::UserRepoTrait};

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
        fn create_user(&mut self, _: User) {
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

        fn find_user(&mut self, name: String) -> Option<User> {
            todo!()
        }
    }
}
