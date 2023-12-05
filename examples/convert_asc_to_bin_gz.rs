use flate2::{read::GzDecoder, write::GzEncoder};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use japan_geoid::{Geoid, MemoryGrid};

fn main() -> std::io::Result<()> {
    let (lng, lat) = (138.2839817085188, 37.12378643088312);

    // Load from the original ascii format.
    let file = File::open("./gsigeo2011_ver2_2.asc")?;
    let mut reader = BufReader::new(file);
    let geoid = MemoryGrid::from_ascii_reader(&mut reader)?;

    let z = geoid.get_height(lng, lat);
    println!("Input: (lng: {}, lat: {}) -> Geoid height: {}", lng, lat, z);

    // Dump as the binary format.
    let file = File::create("./gsigeo2011_ver2_2.bin.gz")?;
    let mut writer = GzEncoder::new(BufWriter::new(file), flate2::Compression::fast());
    geoid.to_binary_writer(&mut writer)?;
    writer.finish()?;

    // Load from the binary.
    let mut file = File::open("./gsigeo2011_ver2_2.bin.gz")?;
    let mut reader = GzDecoder::new(&mut file);
    let geoid = MemoryGrid::from_binary_reader(&mut reader)?;

    let z = geoid.get_height(lng, lat);
    println!("Input: (lng: {}, lat: {}) -> Geoid height: {}", lng, lat, z);

    Ok(())
}
