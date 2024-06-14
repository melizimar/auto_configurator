use crate::core::entities::user_entity::User;

pub struct UserService;

impl UserService {

    pub fn create_user(user_type: &str, username: &str, password: &str) -> User {
        User::new(user_type, username, password)
    }
    /* 
    pub fn create_users(users: Vec<User>) {
        for user in users {
            self::create_user(user.user_type, user.username, user.password);
        }
    }
    */
}
