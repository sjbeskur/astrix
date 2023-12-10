mod gaia_reader;
mod star;
mod trig;
mod star_pair;

pub mod tile_strategy;
pub use gaia_reader::*;
pub use star::*;
use std::error::Error;
pub use trig::*;
pub use star_pair::*;

type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
