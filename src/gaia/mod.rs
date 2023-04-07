mod reader;
mod star;

pub use reader::*;
pub use star::*;
use std::error::Error;

type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
