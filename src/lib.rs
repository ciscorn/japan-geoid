pub mod gsi;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

/// Geoid model
pub trait Geoid {
    fn get_height(&self, lng: f64, lat: f64) -> f64;
}
