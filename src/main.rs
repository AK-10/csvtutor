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
// 指定したレコードの型が、実際のそれぞれのレコードの順序と一致している必要がある
type Record = (String, String, Option<u64>, f64, f64);

// Box<T>はヒープのデータを指す
// https://doc.rust-jp.rs/book/second-edition/ch15-01-box.html
fn run() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.deserialize() {
        // must tell Serde what type we want to deserialize into.
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

// fn get_first_arg() -> Result<OsString, Box<Error>> {
//     match env::args_os().nth(1) {
//         None => Err(From::from("expected 1 argument, but got none")),
//         Some(file_path) => Ok(file_path)
//     }
// }
