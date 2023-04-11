mod reader;
mod star;
mod trig;

pub use reader::*;
pub use star::*;
use std::error::Error;
pub use trig::*;

type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
