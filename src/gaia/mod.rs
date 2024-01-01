mod gaia_reader;
mod star;
mod trig;
mod star_pair;

pub mod db;
pub mod tile_strategy;

pub use gaia_reader::*;
pub use star::*;
pub use trig::*;
pub use star_pair::*;

use std::error::Error;
type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
