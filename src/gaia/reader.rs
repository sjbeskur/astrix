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

	pub fn read_csv(mut self, sender: std::sync::mpsc::Sender<Star>) -> AppResult<()> {
		trace!("read_csv: {:?}", self.file_name);
		// Build the CSV reader and iterate over each record.
		//let mut rdr = csv::Reader::from_reader(File::open(&self.file_name)?);
		let _headers = self.reader.headers().unwrap().clone();
		let iter = self.reader.into_deserialize();
		let mut counter = 0;
		for (i, result) in iter.enumerate() {
			let record: Star = result?;
			counter = i;
			sender.send(record)?;
		}
		info!("Number of records: {}", counter + 1);
		Ok(())
	}
}
