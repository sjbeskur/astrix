use astrix::gaia::*;
use std::sync::mpsc::channel;

fn main() {
    let file_name = "data/1680058493237O-result.csv";

    let (rx, tx) = channel();

    let reader = GaiaFileReader::new(file_name.into());
    reader.read_csv(rx);
}
