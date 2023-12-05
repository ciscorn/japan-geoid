use std::{
    fs::File,
    io::{BufReader, Read, Write},
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
    let mut writer = std::io::Cursor::new(Vec::new());
    geoid.to_binary_writer(&mut writer)?;
    File::create("./gsigeo2011_ver2_2.bin.lz4")?
        .write_all(&lz4_flex::compress_prepend_size(&writer.into_inner()))?;

    // Load from the binary.
    let mut file = File::open("./gsigeo2011_ver2_2.bin.lz4")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let decompressed = &lz4_flex::decompress_size_prepended(&buf).unwrap();
    let mut reader = std::io::Cursor::new(decompressed);
    let geoid = MemoryGrid::from_binary_reader(&mut reader)?;

    let z = geoid.get_height(lng, lat);
    println!("Input: (lng: {}, lat: {}) -> Geoid height: {}", lng, lat, z);

    Ok(())
}
