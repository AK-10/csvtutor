// csv crateをプログラムから利用可能にする
extern crate csv;

use std::error::Error;
use std::io;
use std::process;

pub fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

// Box<T>はヒープのデータを指す
// https://doc.rust-jp.rs/book/second-edition/ch15-01-box.html
fn run() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // `?`はmatchのシンタックスシュガー
        // matchより効果的である
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
