use crate::models::models::User;

pub trait UserControllerTrait: Sync + Send {
    fn add_user(&mut self, user: User);
    fn get_user(&mut self, id: i32) -> Result<User, Box<dyn std::error::Error>>;
    fn list_users(&mut self) -> Vec<User>;
    fn remove_user(&mut self, id: i32);
}
