extern crate csv;

use std::error::Error;
use std::io;
use std::process;

fn run() -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;

    // serializeで基本的なRustの値をレコードに書き込むことができる
    // None::<u64>について
    // シリアライズを行うために具体的な型が必要である
    // None::<u64>はOption<u64>型である
    wtr.serialize(("Davidsons Landing", "AK", None::<u64>, 65.2419444, -165.2716667))?;
    wtr.serialize(("Kenai", "AK", Some(7610), 60.5544444, -151.2583333))?;
    wtr.serialize(("Oakman", "AL", None::<u64>, 33.7133333, -87.3886111))?;

    wtr.flush()?;

    Ok(())
}

pub fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
