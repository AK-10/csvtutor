extern crate csv;

use std::error::Error;
use std::io;
use std::process;

fn run() -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // 手動でレコードを書き出す時は，明示的にヘッダを書く必要がある
    // ヘッダのレコードは他のレコード（値のレコード）と同じように書き出される
    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

    // write_recordの定義
    // pub fn write_record<I, T>(&mut self, record: I) -> csv::Result<()>
    //     where I: IntoIterator<Item=T>, T: AsRef<[u8]>
    // {
    //     // 実装は省略
    // }
    // selfはWriter自身
    // 引数recordはI型
    // IはIntoIterator<Item=T>境界によって制限されており
    // IはIntoIteratorを満足する実装を持つ必要がある
    // Tの値を生じるイテレータを欲している
    // ここでTは書き込みを行いたいそれぞれのフィールドの型を表す
    // TはAsRef<[u8]>境界で制限されている
    // AsRef traitはRustにおいてゼロコストな変換を記述する方法
    // AsRef<[u8]>における[u8]はTからバイト列のスライスをborrowすることができることを意味する
    // AsRef<[u8]>境界はString, &str, Vec<u8>のような型が全て条件を満たす型として当てはまる
    // csv::Result<()>はResult<(), csv::Error>のエイリアス

    // CSV writerが内部で持っているバッファにrecordをためている
    // wtr.flush()でバッファないのレコードを吐き出す
    wtr.flush()?;

    Ok(())
}

pub fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
