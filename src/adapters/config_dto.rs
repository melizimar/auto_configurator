use crate::core::entities::user_entity::User;
use crate::core::entities::application_entity::Application;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub apps: Vec<Application>,
    pub users: Vec<User>,
}