use crate::models::models::User;

pub trait UserRepoTrait: Send + Sync {
    fn create_user(&mut self, user: User);
    fn get_user(&mut self, id: i32) -> Result<User, Box<dyn std::error::Error>>;
    fn find_user(&mut self, name: String) -> Option<User>;
    fn list_users(&mut self) -> Vec<User>;
    fn remove_user(&mut self, id: i32);
}
