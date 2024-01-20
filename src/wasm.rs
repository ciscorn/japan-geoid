use wasm_bindgen::prelude::*;

use crate::gsi;
use crate::gsi::MemoryGrid;
use crate::Geoid;

#[wasm_bindgen]
pub struct GsiGeoid {
    geoid: MemoryGrid<'static>,
}

#[wasm_bindgen]
impl GsiGeoid {
    #[wasm_bindgen]
    pub fn get_height(&self, lng: f64, lat: f64) -> f64 {
        self.geoid.get_height(lng, lat)
    }

    #[wasm_bindgen]
    pub fn get_heights(&self, lngs: &[f64], lats: &[f64]) -> Vec<f64> {
        lngs.iter()
            .zip(lats.iter())
            .map(|(lng, lat)| self.geoid.get_height(*lng, *lat))
            .collect()
    }
}

#[wasm_bindgen]
pub fn load_embedded_gsigeo2011() -> GsiGeoid {
    GsiGeoid {
        geoid: gsi::load_embedded_gsigeo2011(),
    }
}
