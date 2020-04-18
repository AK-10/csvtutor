// csv crateをプログラムから利用可能にする
extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::process;
use std::io;

// Debugを継承する必要はないが，習慣としてどの型に対してもつけると良い.
// field name がCSVの列順と同じではないことに注意されたい.
// csv側
// City,State,Population,Latitude,Longitude
// Record側
// latitude,longitude,population,city,state
// #[serde(rename_all = "PascalCase")]でRecordで定義されているそれぞれのfieldを
// パース時にキャピタライズしてfield名を合わせている
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    // deserializeに失敗した場合，全てNoneとして扱う
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<f64>,
    city: String,
    state: String,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

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
