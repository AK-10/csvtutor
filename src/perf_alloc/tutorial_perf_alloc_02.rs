extern crate csv;

use std::error::Error;
use std::io;
use std::process;

fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let mut count = 0;
    // byte_record()はByteRecordIterを返す
    // &[u8]でフィールドへのアクセスを行う
    for result in rdr.byte_records() {
        let record = result?;
        if &record[0] == b"us" && &record[3] == b"MA" {
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
