use clap::{App, Arg};
use std::error::Error;
use std::io::{BufWriter, Read};
use std::{io, process};

struct Cli {
    delimiter: u8,
    has_header: bool,
}

fn from_args() -> Cli {
    let mut args: Cli = Cli {
        delimiter: 9,
        has_header: true,
    };
    let matches = App::new("csv2md")
        .version("1.0.0")
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .about("(option) delimiter")
                .takes_value(true)
                .default_value("\t"),
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
    args.delimiter = matches.value_of("delimiter").unwrap_or("\t").as_bytes()[0];
    args.has_header = match matches.value_of("header") {
        Some(header) => header == "true",
        None => true,
    };
    args
}

fn run(data: &[u8], args: Cli, writer: impl std::io::Write) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(args.delimiter)
        .has_headers(args.has_header)
        .from_reader(data);
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'|')
        .from_writer(writer);
    if args.has_header {
        let header = reader.headers()?;
        writer.write_record(header)?;
        let len = header.len();
        let header_line = vec![" -- "; len];
        writer.write_record(&header_line)?;
    }
    for line in reader.records() {
        let record = line?;
        writer.write_record(&record)?;
    }
    writer.flush()?;
    Ok(())
}

fn main() {
    let mut data = vec![];
    if let Err(err) = io::stdin().read_to_end(&mut data) {
        eprintln!("{}", err);
        process::exit(1);
    }
    let args = from_args();
    let stdout = io::stdout();
    let out = BufWriter::new(stdout.lock());
    if let Err(err) = run(&data, args, out) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use crate::{run, Cli};

    #[test]
    fn delimiter_comma_test() {
        let data = "\
city,country,pop
Boston,United States,4628910
Concord,United States,42695
";
        let markdown = "city|country|pop\n -- | -- | -- \nBoston|United States|4628910\nConcord|United States|42695\n";
        let args = Cli {
            delimiter: ",".as_bytes()[0],
            has_header: true,
        };
        let mut writer = Vec::new();
        let result = run(data.as_bytes(), args, &mut writer);
        assert!(result.is_ok());
        assert_eq!(writer, markdown.as_bytes());
    }
}
