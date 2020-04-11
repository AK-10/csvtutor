// csv crateをプログラムから利用可能にする
extern crate csv;

use std::io;
use std::process;

pub fn main() {
    // Create a csv parser
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // loop over each record
    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("error reading CSV from <stdin>: {}", err);
                process::exit(1);
            }
        }
    }
}
