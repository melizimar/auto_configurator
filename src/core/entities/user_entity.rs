use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_type: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(user_type: &str, username: &str, password: &str) -> Self {
        User {
            user_type: String::from(user_type),
            username: String::from(username),
            password: String::from(password)
        }
    }
}

//pub struct Config {
//    pub winget_apps: Vec<String>,
//    pub users: Vec<User>,
//    pub font_folder: String,
//}