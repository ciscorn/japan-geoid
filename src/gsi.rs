use crate::Geoid;
use std::borrow::Cow;
use std::io::{self, BufRead, Read, Write};

/// Gridded geoid model
pub trait Grid {
    fn grid_info(&self) -> &GridInfo;
    fn lookup_grid_points(&self, ix: u32, iy: u32) -> f64;

    #[inline]
    fn get_interpolated_value(&self, x: f64, y: f64) -> f64 {
        use std::f64::NAN;
        let grid = self.grid_info();
        let grid_x = (x - grid.x_min as f64) * (grid.x_denom as f64);
        let grid_y = (y - grid.y_min as f64) * (grid.y_denom as f64);
        if grid_x < 0.0 || grid_y < 0.0 {
            return NAN;
        }

        let ix = grid_x.floor() as u32;
        let iy = grid_y.floor() as u32;
        let x_residual = grid_x - ix as f64;
        let y_residual = grid_y - iy as f64;

        if ix >= grid.x_num || iy >= grid.y_num {
            NAN
        } else {
            let lookup_or_nan = |x, y, cond: bool| {
                if cond {
                    self.lookup_grid_points(x, y)
                } else {
                    NAN
                }
            };

            bilinear(
                x_residual,
                y_residual,
                self.lookup_grid_points(ix, iy),
                lookup_or_nan(ix + 1, iy, ix < grid.x_num - 1),
                lookup_or_nan(ix, iy + 1, iy < grid.y_num - 1),
                lookup_or_nan(ix + 1, iy + 1, ix < grid.x_num - 1 && iy < grid.y_num - 1),
            )
        }
    }
}

/// Bilinear interpolation
fn bilinear(x: f64, y: f64, v00: f64, v01: f64, v10: f64, v11: f64) -> f64 {
    if x == 0.0 && y == 0.0 {
        v00
    } else if x == 0.0 {
        v00 * (1.0 - y) + v10 * y
    } else if y == 0.0 {
        v00 * (1.0 - x) + v01 * x
    } else {
        v00 * (1.0 - x) * (1.0 - y) + v01 * x * (1.0 - y) + v10 * (1.0 - x) * y + v11 * x * y
    }
}

/// Grid parameters
#[derive(Debug)]
pub struct GridInfo {
    /// Number of grid points along X-axis
    x_num: u32,
    /// Number of grid points along Y-axis
    y_num: u32,
    /// Denominator of grid interval along X-axis
    x_denom: u32,
    /// Denominator of grid interval along Y-axis
    y_denom: u32,
    /// Minimum value of X-axis
    x_min: f32,
    /// Minimum value of Y-axis
    y_min: f32,
    /// ikind (not used)
    ikind: u16,
    /// Version
    version: String,
}

/// In-memory gridded geoid model
#[derive(Debug)]
pub struct MemoryGrid<'a> {
    pub grid_info: GridInfo,
    points: Cow<'a, [i32]>,
}

impl<'a> Grid for MemoryGrid<'a> {
    /// Gets grid parameters
    fn grid_info(&self) -> &GridInfo {
        &self.grid_info
    }

    /// Gets the value of the grid point at (ix, iy)
    #[inline]
    fn lookup_grid_points(&self, ix: u32, iy: u32) -> f64 {
        match self.points[(self.grid_info.x_num * iy + ix) as usize] {
            9990000 => f64::NAN,
            v => v as f64 / 10000.0,
        }
    }
}

impl<'a> Geoid for MemoryGrid<'a> {
    /// Gets the height of the geoid at (lng, lat)
    #[inline]
    fn get_height(&self, lng: f64, lat: f64) -> f64 {
        self.get_interpolated_value(lng, lat)
    }
}

impl<'a> MemoryGrid<'a> {
    /// Loads the geoid model from a binary file.
    pub fn from_binary_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        // Read header
        let mut buf = [0; 28];
        reader.read_exact(&mut buf)?;
        let grid_info = GridInfo {
            x_num: u16::from_le_bytes(buf[0..2].try_into().unwrap()) as u32,
            y_num: u16::from_le_bytes(buf[2..4].try_into().unwrap()) as u32,
            x_denom: u16::from_le_bytes(buf[4..6].try_into().unwrap()) as u32,
            y_denom: u16::from_le_bytes(buf[6..8].try_into().unwrap()) as u32,
            x_min: f32::from_le_bytes(buf[8..12].try_into().unwrap()),
            y_min: f32::from_le_bytes(buf[12..16].try_into().unwrap()),
            ikind: u16::from_le_bytes(buf[16..18].try_into().unwrap()),
            version: String::from_utf8_lossy(&buf[18..28]).into(),
        };

        // Read grid point values
        let mut points = Vec::with_capacity((grid_info.x_num * grid_info.y_num) as usize);
        let mut buf = [0; 4];
        let mut prev_x1y1 = 9990000;
        let mut prev_x1 = 9990000;
        for pos in 0..(grid_info.y_num * grid_info.x_num) as usize {
            // linear prediction
            let prev_y1 = match pos {
                _ if pos < grid_info.x_num as usize => 9990000,
                _ => points[pos - grid_info.x_num as usize],
            };
            reader.read_exact(&mut buf)?;
            let predicted = prev_x1 + prev_y1 - prev_x1y1;
            let curr = predicted + i32::from_le_bytes(buf);
            points.push(curr);
            (prev_x1, prev_x1y1) = (curr, prev_y1);
        }

        Ok(MemoryGrid {
            grid_info,
            points: points.into(),
        })
    }

    /// Dumps the geoid model to a binary file.
    pub fn to_binary_writer<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        // Write header
        writer.write_all(&(self.grid_info.x_num as u16).to_le_bytes())?;
        writer.write_all(&(self.grid_info.y_num as u16).to_le_bytes())?;
        writer.write_all(&(self.grid_info.x_denom as u16).to_le_bytes())?;
        writer.write_all(&(self.grid_info.y_denom as u16).to_le_bytes())?;
        writer.write_all(&self.grid_info.x_min.to_le_bytes())?;
        writer.write_all(&self.grid_info.y_min.to_le_bytes())?;
        writer.write_all(&(self.grid_info.ikind).to_le_bytes())?;
        if self.grid_info.version.len() > 10 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "version string must be shorter than 10 characters",
            ));
        }
        writer.write_all(self.grid_info.version.as_bytes())?;
        for _ in 0..10 - self.grid_info.version.len() {
            writer.write_all(&[0])?;
        }

        // Write grid point values
        let mut prev_x1y1 = 9990000;
        let mut prev_x1 = 9990000;
        for pos in 0..(self.grid_info.y_num * self.grid_info.x_num) as usize {
            // linear prediction
            let curr = self.points[pos];
            let prev_y1 = match pos {
                _ if pos < self.grid_info.x_num as usize => 9990000,
                _ => self.points[pos - self.grid_info.x_num as usize],
            };
            let predicted = prev_x1 + prev_y1 - prev_x1y1;
            let d = curr - predicted;
            writer.write_all(&d.to_le_bytes())?;
            (prev_x1, prev_x1y1) = (curr, prev_y1);
        }
        Ok(())
    }

    /// Loads GSI's original geoid model in ASCII format.
    pub fn from_ascii_reader<R: BufRead>(reader: &mut R) -> io::Result<Self> {
        use io::{Error, ErrorKind::InvalidData};
        let mut reader = io::BufReader::new(reader);
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let c: Vec<&str> = line.split_whitespace().collect();
        if c.len() != 8 {
            return Err(Error::new(InvalidData, "header line must have 8 values"));
        }
        if c[2] != "0.016667" {
            return Err(Error::new(
                InvalidData,
                "latitude interval must be 0.016667",
            ));
        }
        if c[3] != "0.025000" {
            return Err(Error::new(
                InvalidData,
                "longitude interval must be 0.025000",
            ));
        }

        let grid_info = GridInfo {
            x_num: c[5]
                .parse()
                .map_err(|_| Error::new(InvalidData, "cannot parse header"))?,
            y_num: c[4]
                .parse()
                .map_err(|_| Error::new(InvalidData, "cannot parse header"))?,
            x_denom: 40,
            y_denom: 60,
            x_min: c[1]
                .parse()
                .map_err(|_| Error::new(InvalidData, "cannot parse header"))?,
            y_min: c[0]
                .parse()
                .map_err(|_| Error::new(InvalidData, "cannot parse header"))?,
            ikind: c[6]
                .parse()
                .map_err(|_| Error::new(InvalidData, "cannot parse header"))?,
            version: c[7].to_string(),
        };

        let mut points = Vec::with_capacity((grid_info.x_num * grid_info.y_num) as usize);
        for line_or_err in reader.lines() {
            match line_or_err {
                Ok(line) => {
                    for s in line.split_ascii_whitespace() {
                        let s = s.replace('.', "");
                        let Ok(n) = s.parse::<i32>() else {
                            return Err(Error::new(InvalidData, "Invalid data"));
                        };
                        points.push(n);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(MemoryGrid {
            grid_info,
            points: points.into(),
        })
    }
}

/// Loads the embedded GSIGEO2011 Japan geoid model.
///
/// ```
/// use japan_geoid::gsi::load_embedded_gsigeo2011;
/// use japan_geoid::Geoid;
///
/// let geoid = load_embedded_gsigeo2011();
/// let height = geoid.get_height(138.2839817085188, 37.12378643088312);
/// assert!((height - 39.473870927576634).abs() < 1e-6)
/// ```
pub fn load_embedded_gsigeo2011() -> MemoryGrid<'static> {
    const EMBEDDED_MODEL: &[u8] = include_bytes!("gsigeo2011_ver2_2.bin.lz4");
    MemoryGrid::from_binary_reader(&mut std::io::Cursor::new(
        lz4_flex::decompress_size_prepended(EMBEDDED_MODEL).unwrap(),
    ))
    .unwrap()
}

/// Loads the embedded GSIGEO2024 Japan geoid model.
///
/// ```
/// use japan_geoid::gsi::load_embedded_gsigeo2024;
/// use japan_geoid::Geoid;
///
/// let geoid = load_embedded_gsigeo2024();
/// let height = geoid.get_height(138.2839817085188, 37.12378643088312);
/// assert!((height - 39.61005876363226).abs() < 1e-6)
/// ```
pub fn load_embedded_gsigeo2024() -> MemoryGrid<'static> {
    const EMBEDDED_MODEL: &[u8] = include_bytes!("gsigeo2024_beta.bin.lz4");
    MemoryGrid::from_binary_reader(&mut std::io::Cursor::new(
        lz4_flex::decompress_size_prepended(EMBEDDED_MODEL).unwrap(),
    ))
    .unwrap()
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::fs::File;
    use std::io::{BufReader, Cursor};

    use super::*;

    #[test]
    fn embedded2011() {
        let geoid = load_embedded_gsigeo2011();
        let _ = format!("{:?}", geoid);

        // Compare with the result of PROJ
        let height = geoid.get_height(138.2839817085188, 37.12378643088312);
        assert!((height - 39.473870927576634).abs() < 1e-6);

        // Compare with the result of GSI's online calculator
        // https://vldb.gsi.go.jp/sokuchi/surveycalc/geoid/calcgh/calc_f.html
        let height = geoid.get_height(141.12345, 43.12345);
        assert!((height - 32.8389).abs() < 1e-4);
        let height = geoid.get_height(127.6791822004209, 26.212208125371717);
        assert!((height - 31.4807).abs() < 1e-4);

        let height = geoid.get_height(10.0, 10.0);
        assert!(f64::is_nan(height));

        let height = geoid.get_height(120.0, 20.0);
        assert!(f64::is_nan(height));

        let height = geoid.get_height(120.0, 30.0);
        assert!(f64::is_nan(height));

        let height = geoid.get_height(130.0, 20.0);
        assert!(f64::is_nan(height));

        let height = geoid.get_height(130.0, 20.0);
        assert!(f64::is_nan(height));

        let height = geoid.get_height(130.0, 60.0);
        assert!(f64::is_nan(height));

        let height = geoid.get_height(150.0, 30.0);
        assert!(f64::is_nan(height));

        let info = geoid.grid_info();
        let _ = format!("{:?}", info);
        assert_eq!(info.x_num, 1201);
        assert_eq!(info.y_num, 1801);
        assert_eq!(info.version, "ver2.2\0\0\0\0");
        assert_eq!(info.x_denom, 40);
        assert_eq!(info.y_denom, 60);
        assert_eq!(info.x_min, 120.0);
        assert_eq!(info.y_min, 20.0);
    }

    #[test]
    fn embedded2024() {
        let geoid = load_embedded_gsigeo2024();
        let _ = format!("{:?}", geoid);

        let height = geoid.get_height(138.2839817085188, 37.12378643088312);
        assert!((height - 39.61005876363226).abs() < 1e-6);

        // compare with the results of GSI's 'geoidcalc' implementation
        let height = geoid.get_height(140.085365000, 36.104394000);
        assert!((height - 40.3059).abs() < 1e-4);
        let height = geoid.get_height(139.615526456, 35.160410123);
        assert!((height - 36.7568).abs() < 1e-4);
        let height = geoid.get_height(138.215695342, 36.832842854);
        assert!((height - 41.6041).abs() < 1e-4);
        let height = geoid.get_height(130., 30.);
        assert!((height - 30.5669).abs() < 1e-4);

        let height = geoid.get_height(120.0, 15.0);
        assert!(!f64::is_nan(height));
        let height = geoid.get_height(120.0, 50.0);
        assert!(!f64::is_nan(height));
        let height = geoid.get_height(160.0, 15.0);
        assert!(!f64::is_nan(height));
        let height = geoid.get_height(160.0, 50.0);
        assert!(!f64::is_nan(height));

        let height = geoid.get_height(130.0, 15.0);
        assert!(!f64::is_nan(height));
        let height = geoid.get_height(130.0, 50.0);
        assert!(!f64::is_nan(height));
        let height = geoid.get_height(120.0, 20.0);
        assert!(!f64::is_nan(height));
        let height = geoid.get_height(160.0, 20.0);
        assert!(!f64::is_nan(height));

        let height = geoid.get_height(130.0, 14.99);
        assert!(f64::is_nan(height));
        let height = geoid.get_height(130.0, 50.01);
        assert!(f64::is_nan(height));
        let height = geoid.get_height(119.99, 20.0);
        assert!(f64::is_nan(height));
        let height = geoid.get_height(160.01, 20.0);
        assert!(f64::is_nan(height));

        let info = geoid.grid_info();
        let _ = format!("{:?}", info);
        assert_eq!(info.x_num, 1601);
        assert_eq!(info.y_num, 2101);
        assert_eq!(info.version, "ver-beta\0\0");
        assert_eq!(info.x_denom, 40);
        assert_eq!(info.y_denom, 60);
        assert_eq!(info.x_min, 120.0);
        assert_eq!(info.y_min, 15.0);
    }

    #[test]
    fn ascii_to_binary() {
        // from ascii
        let mut reader = BufReader::new(File::open("./tests/dummy-geoid.asc").unwrap());
        let geoid = MemoryGrid::from_ascii_reader(&mut reader).unwrap();

        // to binary
        let mut buffer = Vec::new();
        geoid.to_binary_writer(&mut buffer).unwrap();

        // from binary
        let mut geoid = MemoryGrid::from_binary_reader(&mut Cursor::new(buffer)).unwrap();

        // to binary (broken data)
        let mut buffer = Vec::new();
        geoid.grid_info.version = "ver22222222222222".to_string();
        geoid
            .to_binary_writer(&mut buffer)
            .expect_err("version string must be shorter than 10 characters");
    }

    #[test]
    fn broken_asc_headers() {
        let headers = vec![
            "20.aaa00 120.00000 0.016667 0.025000 1801 1201 1 ver2.2",
            "20.00000 120.0bbb0 0.016667 0.025000 1801 1201 1 ver2.2",
            "20.00000 120.00000 0.116667 0.025000 1801 1201 1 ver2.2",
            "20.00000 120.00000 0.016667 0.225000 1801 1201 1 ver2.2",
            "20.00000 120.00000 0.016667 0.025000 -1801 1201 1 ver2.2",
            "20.00000 120.00000 0.016667 0.025000 1801 -1201 1 ver2.2",
            "20.00000 120.00000 0.016667 0.025000 1801 1201 z ver2.2",
            "20.00000 120.00000 0.016667 0.025000 1801 1201 1 ver2.2 foobar",
            "20.00000 120.00000 0.016667 0.025000 1801 1201 1 ver2.2\n000.000a",
        ];

        for h in headers {
            let Err(err) = MemoryGrid::from_ascii_reader(&mut BufReader::new(Cursor::new(h)))
            else {
                panic!("expected error");
            };
            println!("{:?}", err);
        }
    }
}
