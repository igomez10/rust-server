use crate::models::User;

pub trait UserCtrlTrait {
    fn get_name(&mut self) -> String;
    fn set_name(&mut self, name: String);
    fn add_user(&mut self, user: User);
    fn get_user(&mut self, id: i32) -> Option<User>;
    fn list_users(&mut self) -> Vec<User>;
    fn remove_user(&mut self, id: i32);
}
