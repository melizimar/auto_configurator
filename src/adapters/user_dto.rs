use crate::core::entities::user_entity::User;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub user_type: String,
    pub username: String,
    pub password: String,
}

impl From<UserDTO> for User {
    fn from(dto: UserDTO) -> User {
        User {
            user_type: dto.user_type,
            username: dto.username,
            password: dto.password
        }
    }
}