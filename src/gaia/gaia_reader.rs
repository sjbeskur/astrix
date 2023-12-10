use super::*;
use std::fs::File;
use tracing::{info, trace};

pub struct GaiaFileReader {
	file_name: String,
	reader: csv::Reader<File>,
}

impl GaiaFileReader {
	pub fn new(file_name: String) -> GaiaFileReader {
		Self {
			file_name: file_name.clone(),
			reader: csv::Reader::from_reader(File::open(file_name).unwrap()),
		}
	}

	pub fn read_csv(mut self) -> AppResult<Vec<Star>> {
		trace!("read_csv: {:?}", self.file_name);
		// Build the CSV reader and iterate over each record.
		//let mut rdr = csv::Reader::from_reader(File::open(&self.file_name)?);
		let _headers = self.reader.headers().unwrap().clone();
		let iter = self.reader.into_deserialize();
		let mut counter = 0;
		let mut result_list = Vec::new();

		for record in iter {
			let r : Star = record?;
			result_list.push(r);
		}
		info!("Number of records: {}", counter + 1);
		Ok(result_list)
	}
}
