use std::error::Error;
use std::fs::File;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct GaiaRecord{
    ra: f32,
    dec: f32,
    #[serde(rename = "b1mag")]
    mag: f32,
}

pub struct GaiaFileReader {
    reader: csv::Reader<File>,
}


type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

impl GaiaFileReader {
    pub fn new(file_name: String) -> GaiaFileReader {
        Self {
            reader: csv::Reader::from_reader(File::open(file_name).unwrap()),
        }
    }

    pub fn read_csv(mut self, sender: std::sync::mpsc::Sender<GaiaRecord>) -> AppResult<()> {
        // Build the CSV reader and iterate over each record.
        //let mut rdr = csv::Reader::from_reader(File::open(&self.file_name)?);
        let headers = self.reader.headers().unwrap().clone();
        println!("{:?}",headers);

        let mut iter = self.reader.into_deserialize();

        println!("reading records..");
        let mut i = 0;
        while let Some(result) = iter.next() {
            let record: GaiaRecord = result?;
            i += 1;
            //println!("{:?}",record);
        }

        println!("total recs: {}",i);

        Ok(())
    }

}
