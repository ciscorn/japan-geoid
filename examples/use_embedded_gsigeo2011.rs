use japan_geoid::{Geoid, MemoryGrid};

fn main() {
    // Load the embedded GSIGEO2011 model.
    let geoid = MemoryGrid::from_embedded_gsigeo2011();

    // Calculate the geoid height.
    let (lng, lat) = (138.2839817085188, 37.12378643088312);
    let height = geoid.get_height(lng, lat);
    println!(
        "Input: (lng: {}, lat: {}) -> Geoid height: {}",
        lng, lat, height
    );

    // Returns NaN if the input is outside the domain.
    assert!(f64::is_nan(geoid.get_height(10.0, 10.0)))
}
