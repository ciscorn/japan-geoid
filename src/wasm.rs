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
    #[wasm_bindgen(js_name = "getHeight")]
    pub fn get_height(&self, lng: f64, lat: f64) -> f64 {
        self.geoid.get_height(lng, lat)
    }

    #[wasm_bindgen(js_name = "getHeights")]
    pub fn get_heights(&self, lngs: &[f64], lats: &[f64]) -> Vec<f64> {
        lngs.iter()
            .zip(lats.iter())
            .map(|(lng, lat)| self.geoid.get_height(*lng, *lat))
            .collect()
    }
}

#[wasm_bindgen(js_name = "loadEmbeddedGSIGEO2011")]
pub fn load_embedded_gsigeo2011() -> GsiGeoid {
    GsiGeoid {
        geoid: gsi::load_embedded_gsigeo2011(),
    }
}

/// Load the embedded JPGEO2024 geoid model.
#[wasm_bindgen(js_name = "loadEmbeddedJPGEO2024")]
pub fn load_embedded_jpgeo2024() -> GsiGeoid {
    GsiGeoid {
        geoid: gsi::load_embedded_jpgeo2024(),
    }
}

/// Load the embedded Hrefconv2024 (height reference conversion) model.
#[wasm_bindgen(js_name = "loadEmbeddedHrefconv2024")]
pub fn load_embedded_hrefconv2024() -> GsiGeoid {
    GsiGeoid {
        geoid: gsi::load_embedded_hrefconv2024(),
    }
}

/// Load the embedded JPGEO2024 + Hrefconv2024 combined model.
#[wasm_bindgen(js_name = "loadEmbeddedJPGEO2024Hrefconv2024")]
pub fn load_embedded_jpgeo2024_hrefconv2024() -> GsiGeoid {
    GsiGeoid {
        geoid: gsi::load_embedded_jpgeo2024_hrefconv2024(),
    }
}
