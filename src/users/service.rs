use crate::users::entities::User;

// Cosas de SQLx
impl User {
    pub fn new_user(id: i32, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
        }
    }

    pub fn new_admin(id: i32, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
        }
    }
}
