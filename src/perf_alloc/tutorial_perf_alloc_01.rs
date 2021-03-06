extern crate csv;

use std::error::Error;
use std::io;
use std::process;

fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let mut count = 0;
    // records()はStringRecordsIterを返す
    // &strでフィールドへのアクセスを行う
    for result in rdr.records() {
        let record = result?;
        if &record[0] == "us" && &record[3] == "MA" {
            count += 1;
        }
    }

    Ok(count)
}

pub fn main() {
    match run() {
        Ok(count) => {
            println!("{}", count);
        },
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        },
    }
}
