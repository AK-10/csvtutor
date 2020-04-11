// csv crateをプログラムから利用可能にする
extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::io;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

// Box<T>はヒープのデータを指す
// https://doc.rust-jp.rs/book/second-edition/ch15-01-box.html
fn run() -> Result<(), Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record)
    }

    Ok(())
}

// fn get_first_arg() -> Result<OsString, Box<Error>> {
//     match env::args_os().nth(1) {
//         None => Err(From::from("expected 1 argument, but got none")),
//         Some(file_path) => Ok(file_path)
//     }
// }
