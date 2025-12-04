use std::fs::File;
use std::io::{BufReader, Write};

use japan_geoid::gsi::MemoryGrid;
use japan_geoid::Geoid;

fn convert_isg_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    println!("Converting {input_path} -> {output_path}");

    // Load from ISG format
    let mut reader = BufReader::new(File::open(input_path)?);
    let geoid = MemoryGrid::from_isg_reader(&mut reader)?;

    println!("  Grid info: {:?}", geoid.grid_info);

    // Test with a sample point (Tokyo)
    let (lng, lat) = (139.6917, 35.6895);
    let height = geoid.get_height(lng, lat);
    println!("  Sample point (Tokyo): ({lng}, {lat}) -> {height}");

    // Dump as the efficient binary format with LZ4 compression
    let mut buf = Vec::new();
    geoid.to_binary_writer(&mut buf)?;
    let compressed = lz4_flex::compress_prepend_size(&buf);
    File::create(output_path)?.write_all(&compressed)?;

    println!(
        "  Compressed: {} bytes -> {} bytes ({:.1}%)",
        buf.len(),
        compressed.len(),
        compressed.len() as f64 / buf.len() as f64 * 100.0
    );

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Convert all ISG files
    let files = [
        ("./data/JPGEO2024.isg", "./src/jpgeo2024.bin.lz4"),
        ("./data/Hrefconv2024.isg", "./src/hrefconv2024.bin.lz4"),
        (
            "./data/JPGEO2024+Hrefconv2024.isg",
            "./src/jpgeo2024_hrefconv2024.bin.lz4",
        ),
    ];

    for (input, output) in files {
        convert_isg_file(input, output)?;
        println!();
    }

    println!("All conversions completed!");
    Ok(())
}
