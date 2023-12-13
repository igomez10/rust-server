use crate::models::models::User;
// trait
pub trait UserHandlerTrait: Send + Sync {
    fn add_user(&self, user: User);
    fn get_user(&self, id: i32) -> Option<User>;
    fn list_users(&self) -> Vec<User>;
    fn remove_user(&self, id: i32);
}
