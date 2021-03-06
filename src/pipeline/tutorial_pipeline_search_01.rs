extern crate csv;

use std::env;
use std::error::Error;
use std::io;
use std::process;

fn run() -> Result<(), Box<Error>> {
    // クエリを固定引数として受け取る
    let query = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(query) => query,
    };

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers()?)?;

    // rdr上のレコードを全て舐め，queryを含むレコードをwtrに書き込む
    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == &query) {
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
