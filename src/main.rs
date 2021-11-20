use std::{io, process};
use std::error::Error;

fn read() -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read() {
        println!("error running read: {}", err);
        process::exit(1);
    }
}