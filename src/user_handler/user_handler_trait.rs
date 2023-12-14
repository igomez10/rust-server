use std::error::Error;

use crate::models::models::User;
// trait
pub trait UserHandlerTrait: Send + Sync {
    // add_user method that takes a User and returns nothing
    fn add_user(&self, user: User);
    // get_user method that takes an i32 and returns an Option<User>
    // it returns an error if not found
    fn get_user(&self, id: i32) -> Result<User, Box<dyn Error>>;
    fn list_users(&self) -> Vec<User>;
    fn remove_user(&self, id: i32);
}
