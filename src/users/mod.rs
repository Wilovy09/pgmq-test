pub mod entities {
    mod user;
    pub use user::*;
}

mod dtos;
pub use dtos::*;

pub mod errors;

mod routes;
pub use routes::config as routes;

mod service;
