use clap::{App, Arg};
use std::io;

fn main() {
    let matches = App::new("csv2md")
        .version("1.0.0")
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .about("(option) delimiter")
                .takes_value(true)
                .default_value("\\t"),
        )
        .arg(
            Arg::new("header")
                .short('H')
                .long("header")
                .about("(option) is header exists (true|false)")
                .takes_value(true)
                .default_value("true"),
        )
        .get_matches();
    let delimiter = matches.value_of("delimiter").unwrap();
    let exist_header: bool = match matches.value_of("header") {
        Some(header) => header == "true",
        None => true,
    };
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter.as_bytes()[0])
        .has_headers(exist_header)
        .from_reader(io::stdin());
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'|')
        .from_writer(io::stdout());
    if exist_header {
        let header = reader.headers().unwrap();
        writer.write_record(header).unwrap();
        let len = header.len();
        let mut header_line = Vec::with_capacity(len);
        for _ in 0..len {
            header_line.push(" -- ");
        }
        writer.write_record(&header_line).unwrap();
    }
    for line in reader.records() {
        let record = line.unwrap();
        writer.write_record(&record).unwrap();
    }
    writer.flush().unwrap();
}
