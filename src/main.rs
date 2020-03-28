// csv crateをプログラムから利用可能にする
extern crate csv;

use std::error::Error;
use std::io;
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

// trait objects without an explicit dyn are deprecated
fn run() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
        // ?: matchのシンタックスシュガー Result を返す関数内でのみ使える
        let record = result?;
        println!("{:?}", record);
        // match result {
        //     Err(err) => return Err(From::from(err)),
        //     Ok(record) => {
        //         println!("{:?}", record);
        //     }
        // }
    }

    Ok(())
}

