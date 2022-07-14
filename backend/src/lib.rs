mod error;
pub mod planetscale;

pub use error::*;

use std::result;

pub type Result<T> = result::Result<T, Error>;
