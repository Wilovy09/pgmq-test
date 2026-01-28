mod entities {
    mod email;
    pub use email::*;
}

mod dtos;
pub use dtos::*;

pub mod errors;

mod service;
pub use service::*;
