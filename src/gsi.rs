use std::borrow::Cow;
use std::io::{self, BufRead, Read, Write};

#[derive(Debug)]
pub struct GsiGridInfo {
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

#[derive(Debug)]
pub struct MemoryGrid<'a> {
    pub grid_info: GsiGridInfo,
    points: Cow<'a, [i32]>,
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

pub trait Geoid {
    fn get_height(&self, lng: f64, lat: f64) -> f64;
}

pub trait Grid {
    fn grid_info(&self) -> &GsiGridInfo;
    fn lookup_grid_points(&self, ix: u32, iy: u32) -> f64;

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
        let x_residual = grid_x % ix as f64;
        let y_residual = grid_y % iy as f64;

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

impl<'a> Grid for MemoryGrid<'a> {
    fn grid_info(&self) -> &GsiGridInfo {
        &self.grid_info
    }
    fn lookup_grid_points(&self, ix: u32, iy: u32) -> f64 {
        match self.points[(self.grid_info.x_num * iy + ix) as usize] {
            9990000 => f64::NAN,
            v => v as f64 / 10000.0,
        }
    }
}

impl<'a> Geoid for MemoryGrid<'a> {
    fn get_height(&self, lng: f64, lat: f64) -> f64 {
        self.get_interpolated_value(lng, lat)
    }
}

impl<'a> MemoryGrid<'a> {
    pub fn from_binary_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        // Read header
        let mut buf = [0; 28];
        reader.read_exact(&mut buf)?;
        let grid_info = GsiGridInfo {
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
        while points.len() < (grid_info.y_num * grid_info.x_num) as usize {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            points.push(i32::from_le_bytes(buf));
        }

        Ok(MemoryGrid {
            grid_info,
            points: points.into(),
        })
    }

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
            panic!("version string must be less than 10 characters");
        }
        writer.write_all(self.grid_info.version.as_bytes())?;
        for _ in 0..10 - self.grid_info.version.len() {
            writer.write_all(&[0])?;
        }

        // Write grid point values
        for p in self.points.iter() {
            writer.write_all(&p.to_le_bytes())?;
        }
        Ok(())
    }

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

        let grid_info = GsiGridInfo {
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
