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
        // Resultで結果を検証する
        // 問題なければレコードを標準出力
        // そうでなければエラーメッセージを標準出力して，プログラムを終了する
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                println!("{:?}", record);
            }
        }
    }

    Ok(())
}
