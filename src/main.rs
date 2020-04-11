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
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // lifetimeを明確にするためのスコープ
    // rdr.haedersはborrowを返す
    // CSVリーダーのヘッダからのborrowと、CSVリーダーがレコード上をイテレートしようとするとき必要なborrowを、同時に行うことはできない
    // とあったが，ここはなくてもコンパイルされた
    // https://doc.rust-jp.rs/book/second-edition/ch04-02-references-and-borrowing.html
    // https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/references-and-borrowing.html
    {
        let headers = rdr.headers()?;
        println!("{:?}", headers);
    }


    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record)
    }

    // 原文
    // We can ask for the headers at any time. There's no need to nest this
    // call in its own scope because we never try to borrow the reader again.
    // これ以降reader(rdr)は呼ばれないのでネストする必要はない
    let headers = rdr.headers()?;
    println!("{:?}", headers);

    Ok(())
}

// fn get_first_arg() -> Result<OsString, Box<Error>> {
//     match env::args_os().nth(1) {
//         None => Err(From::from("expected 1 argument, but got none")),
//         Some(file_path) => Ok(file_path)
//     }
// }
