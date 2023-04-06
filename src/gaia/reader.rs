use std::error::Error;
use std::f64::consts::PI;
use std::fs::File;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct GaiaRecord {
    ra: f64,
    dec: f64,
    #[serde(rename = "b1mag")]
    mag: f32,
}

pub struct GaiaFileReader {
    reader: csv::Reader<File>,
}

impl GaiaRecord {
    pub fn new(ra: f64, dec: f64, mag: f32) -> Self {
        Self { ra, dec, mag }
    }

    pub fn angular_separation(&self, other_star: &GaiaRecord) -> f64 {
        // Calculate the angular separation between the stars
        //let sep_deg = ra1 * 15.0 / 180.0 * PI, dec1 / 180.0 * PI, ra2 * 15.0 / 180.0 * PI, dec2 / 180.0 * PI;
        let ra1 = self.ra.to_radians();
        let dec1 = self.dec.to_radians();
        let ra2 = other_star.ra.to_radians();
        let dec2 = other_star.dec.to_radians();

        // Calculate the cosine of the angular separation
        let cos_theta = (dec1.sin() * dec2.sin()) + (dec1.cos() * dec2.cos() * (ra1 - ra2).cos());

        // Calculate the angular separation in radians
        let theta = cos_theta.acos();

        // Convert the angular separation from radians to degrees
        let sep_deg = theta.to_degrees();

        sep_deg
    }
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
        println!("{:?}", headers);

        let mut iter = self.reader.into_deserialize();

        println!("reading records..");
        let mut i = 0;
        while let Some(result) = iter.next() {
            let record: GaiaRecord = result?;
            i += 1;
            //println!("{:?}",record);
        }

        println!("total recs: {}", i);

        Ok(())
    }
}
