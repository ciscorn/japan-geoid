# japan-geoid

Rust and Python library for calculating geoid heights in Japan using [GSI's geoid model](https://fgd.gsi.go.jp/download/geoid.php).

[国土地理院のジオイドモデル](https://fgd.gsi.go.jp/download/geoid.php)を用いて日本のジオイド高を計算する Rust 用および Python 用のライブラリ。

## Python binding

See [`japan-geoid-py`](./japan-geoid-py/).

## Rust crate

See examples in the [examples](./examples/) directory.

```rust
use flate2::{read::GzDecoder, write::GzEncoder};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use japan_geoid::{Geoid, MemoryGrid};

fn main() -> std::io::Result<()> {
    let (lng, lat) = (138.2839817085188, 37.12378643088312);

    // Load from the original ascii format.
    let geoid =
        MemoryGrid::from_ascii_reader(&mut BufReader::new(File::open("./gsigeo2011_ver2_2.asc")?))?;

    // Calculate the geoid height.
    let height = geoid.get_height(lng, lat);
    println!(
        "Input: (lng: {}, lat: {}) -> Geoid height: {}",
        lng, lat, height
    );

    // Dump as the efficient binary format.
    geoid.to_binary_writer(&mut GzEncoder::new(
        BufWriter::new(File::create("./gsigeo2011_ver2_2.bin.gz")?),
        flate2::Compression::fast(),
    ))?;

    // Load the binary model.
    let geoid = MemoryGrid::from_binary_reader(&mut GzDecoder::new(File::open(
        "./gsigeo2011_ver2_2.bin.gz",
    )?))?;

    let height = geoid.get_height(lng, lat);
    println!(
        "Input: (lng: {}, lat: {}) -> Geoid height: {}",
        lng, lat, height
    );

    Ok(())
}
```
