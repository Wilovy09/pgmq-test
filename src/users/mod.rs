mod entities {
    mod user;
    pub use user::*;
}

mod routes;
pub use routes::config as routes;

mod service;
