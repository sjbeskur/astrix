use astrix::gaia::*;
use std::sync::mpsc::channel;
use clap::{Parser};

mod cli;
fn main() {

    let args = cli::Config::parse();
    let file_name = args.filename;

    let (tx, rx) = channel();

    let reader = GaiaFileReader::new(file_name);
    let _ = reader.read_csv(tx);

    let handle = std::thread::spawn(move || {
        let mut prev: Option<Star> = None;
        while let Ok(result) = rx.recv(){
            let curr = result;
            if let Some(prev) = prev {
                let angle = prev.angular_separation(&curr);
                //println!("{}", angle);
            }
            prev = Some(curr);
        }
    });

    handle.join().unwrap();

}

