extern crate csv;

use std::env;
use std::error::Error;
use std::io;
use std::process;

fn run() -> Result<(), Box<Error>> {
    let query = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(query) => query,
    };

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.byte_headers()?)?;

    for result in rdr.byte_records() {
        let record = result?;
        // queryはString型でfieldは&[u8]型
        // queryを比較できるように&[u8]に変換する必要あり
        if record.iter().any(|field| field == query.as_bytes()) {
            wtr.write_record(&record)?;
        }
    }

    wtr.flush()?;

    Ok(())
}

pub fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
