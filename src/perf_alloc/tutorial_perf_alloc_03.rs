extern crate csv;

use std::error::Error;
use std::io;
use std::process;

fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // メモリの確保をrecordの一つのみにしている
    let mut record = csv::ByteRecord::new();

    let mut count = 0;

    // fn read_byte_record(&mut self, record: &mut ByteRecord) -> csv::Result<bool>;
    // レコードが読み込まれたときにtrue, csvリーダからの入力がなくなったときfalse
    // record: &mut ByteRecordにレコード内容を上書きする
    while rdr.read_byte_record(&mut record)? {
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

