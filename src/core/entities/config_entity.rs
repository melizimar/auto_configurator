use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub winget_apps: Vec<String>,
    pub users: Vec<User>,
    pub font_folder: String,
}

#[derive(Deserialize)]
pub struct User {
    pub user_type: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(user_type: String, username: String, password: String) -> Self {
        User {
            user_type,
            username,
            password
        }
    }
}