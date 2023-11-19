use ndarray::{Array, Zip};
use numpy::*;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::borrow::Cow;

use ::japan_geoid::*;

#[pyclass]
struct GsiGeoid {
    geoid: MemoryGrid<'static>,
}

#[pymethods]
impl GsiGeoid {
    /// Load the geoid model from the original ascii format.
    #[classmethod]
    fn from_ascii(_cls: &PyType, content: &str) -> PyResult<Self> {
        let mut reader = std::io::Cursor::new(content);
        let geoid = MemoryGrid::from_ascii_reader(&mut reader)?;
        Ok(GsiGeoid { geoid })
    }

    /// Load the geoid model from the efficient binary format.
    #[classmethod]
    fn from_binary(_cls: &PyType, content: &[u8]) -> PyResult<Self> {
        let mut reader = std::io::Cursor::new(content);
        let geoid = MemoryGrid::from_binary_reader(&mut reader)?;
        Ok(GsiGeoid { geoid })
    }

    /// Serialize the geoid model in the efficient binary format.
    fn to_binary(&self) -> PyResult<Cow<[u8]>> {
        let mut buf = Vec::new();
        self.geoid.to_binary_writer(&mut buf)?;
        Ok(buf.into())
    }

    /// Get the geoid height at a specified point.
    fn get_height(&self, lng: f64, lat: f64) -> f64 {
        self.geoid.get_height(lng, lat)
    }

    /// Get the geoid height at each specified point.
    fn get_heights<'py>(
        &self,
        py: Python<'py>,
        lng: PyReadonlyArrayDyn<'py, f64>,
        lat: PyReadonlyArrayDyn<'py, f64>,
    ) -> PyResult<&'py PyArrayDyn<f64>> {
        if lng.shape() != lat.shape() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "lng and lat must have the same shape",
            ));
        }
        let mut c = Array::zeros(lng.shape());
        Zip::from(&mut c)
            .and(lng.as_array())
            .and(lat.as_array())
            .for_each(|c, &a, &b| *c = self.geoid.get_height(a, b));
        Ok(c.into_pyarray(py))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn japan_geoid(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GsiGeoid>()?;
    Ok(())
}
