extern crate csv;
extern crate serde;

#[macro_use]
extern create serde_derive

use std::error::Error;
use std::io;
use std::process;

// 構造体は`Serialize`, `Deserialize`の両方を継承できる
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
// <'a>はlifetimeパラメータ
// これは`borrowされたデータを含む`ことを意味する
// この構造体をlifetimeなしで書けるが，&strをownedなString型で置き換えるという事は
// レコードを書き込むたびにcity, stateの新しいStringを
// アロケートしなければならないことを意味する
// これではメモリとパフォーマンスを無駄遣いしてしまう
struct Record<'a> {
    city: &'a str,
    state: &'a str,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn run() -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.serialize(Record {
        city: "Davidsons Landing",
        state: "AK",
        population: Some(7610),
        latitude: 60.5544444,
        longitude: -151.2583333,
    })?;
    wtr.serialize(Record {
        city: "Kenai",
        state: "AK",
        population: Some(7610),
        latitude: 60.5544444,
        longitude: -151.2583333,
    })?;
    wtr.serialize(Record {
        city: "Oakman",
        state: "AL",
        population: None,
        latitude: 33.7133333,
        longitude: -87.3886111,
    })?;

    wtr.flush()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
