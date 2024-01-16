use std::fs::{self, File};
use std::io::{BufReader, Write};

use japan_geoid::{Geoid, MemoryGrid};

fn main() -> std::io::Result<()> {
    let (lng, lat) = (138.2839817085188, 37.12378643088312);

    // Load from the original ascii format.
    let mut reader = BufReader::new(File::open("./gsigeo2011_ver2_2.asc")?);
    let geoid = MemoryGrid::from_ascii_reader(&mut reader)?;

    // Calculate the geoid height.
    let height = geoid.get_height(lng, lat);
    println!(
        "Input: (lng: {}, lat: {}) -> Geoid height: {}",
        lng, lat, height
    );

    // Dump as the efficient binary format.
    let mut buf = Vec::new();
    geoid.to_binary_writer(&mut buf)?;
    File::create("./gsigeo2011_ver2_2.bin.lz4")?
        .write_all(&lz4_flex::compress_prepend_size(&buf))?;

    // Load the binary model.
    let decompressed =
        &lz4_flex::decompress_size_prepended(&fs::read("./gsigeo2011_ver2_2.bin.lz4")?).unwrap();
    let geoid = MemoryGrid::from_binary_reader(&mut std::io::Cursor::new(decompressed))?;

    let height = geoid.get_height(lng, lat);
    println!(
        "Input: (lng: {}, lat: {}) -> Geoid height: {}",
        lng, lat, height
    );

    Ok(())
}
