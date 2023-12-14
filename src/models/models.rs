use serde::{Deserialize, Serialize};
// create structure user that implements ToDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    password: String,
}

pub struct MockUser {}

// interface toDB
pub trait ToDB {
    fn to_db(&self);
}

// create method getName
impl User {
    // constructor
    pub fn new(id: i32, name: String, email: String, password: String) -> User {
        User {
            id,
            name,
            email,
            password,
        }
    }
    pub fn to_db(&self) {
        log::debug!("Saving user to DB");
    }
}

// implement toDB for User
impl ToDB for User {
    fn to_db(&self) {
        log::debug!("Saving user to DB");
    }
}

// implement toDB for MockUser
impl ToDB for MockUser {
    fn to_db(&self) {
        log::debug!("Saving user to Mock DB");
    }
}
