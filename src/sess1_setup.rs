// csv crateをプログラムから利用可能にする
extern crate csv;

use std::io;

pub fn main() {
    // Create a csv parser
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // loop over each record
    for result in rdr.records() {
        let record = result.expect("a CSV record");
        println!("{:?}", record);
    }
}
