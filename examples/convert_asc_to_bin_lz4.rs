//! Convert .asc to .bin.lz4
//!
//! Usage:
//! cargo run --example convert_asc_to_bin_lz4 -- input.asc

use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;

use japan_geoid::gsi::MemoryGrid;
use japan_geoid::Geoid;

fn main() -> std::io::Result<()> {
    let argv = std::env::args().collect::<Vec<_>>();
    let path = Path::new(&argv[1]);

    let (lng, lat) = (138.2839817085188, 37.12378643088312);

    // Load from the original ascii format.
    let mut reader = BufReader::new(File::open(path)?);
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
    let out_path = path.with_extension("bin.lz4");
    File::create(&out_path)?.write_all(&lz4_flex::compress_prepend_size(&buf))?;

    // Load the binary model.
    let decompressed = &lz4_flex::decompress_size_prepended(&fs::read(&out_path)?).unwrap();
    let geoid = MemoryGrid::from_binary_reader(&mut std::io::Cursor::new(decompressed))?;

    let height = geoid.get_height(lng, lat);
    println!(
        "Input: (lng: {}, lat: {}) -> Geoid height: {}",
        lng, lat, height
    );

    Ok(())
}
