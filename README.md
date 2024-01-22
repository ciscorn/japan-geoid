# japan-geoid

[![Test](https://github.com/MIERUNE/japan-geoid/actions/workflows/test.yml/badge.svg)](https://github.com/MIERUNE/japan-geoid/actions/workflows/test.yml)
[![Maturin](https://github.com/MIERUNE/japan-geoid/actions/workflows/maturin.yml/badge.svg)](https://github.com/MIERUNE/japan-geoid/actions/workflows/maturin.yml)
[![codecov](https://codecov.io/gh/MIERUNE/japan-geoid/graph/badge.svg?token=c9T2ayChfw)](https://codecov.io/gh/MIERUNE/japan-geoid)
[![Crates.io Version](https://img.shields.io/crates/v/japan-geoid)](https://crates.io/crates/japan-geoid)
[![PyPI - Version](https://img.shields.io/pypi/v/japan-geoid)](https://pypi.org/project/japan-geoid/)
[![NPM Version](https://img.shields.io/npm/v/japan-geoid)](https://www.npmjs.com/package/japan-geoid)

A library for calculating geoid heights in Japan using GSI's geoid model. It is implemented in Rust and additionally supports Python and JavaScript. The library contains geoid data based on GSIGEO2011 (`gsigeo2011_ver2_2.asc`), created with the permission: 「測量法に基づく国土地理院長承認（使用）R 5JHs 560」.

日本のジオイド高を計算するためライブラリです。Rust で実装されており、Python と JavaScript (WASM) でも利用できます。国土地理院のジオイドモデル「[日本のジオイド2011](https://fgd.gsi.go.jp/download/geoid.php)」を用いて、国土地理院による C++ のサンプルコードに準拠した補間計算を行います。本ライブラリには、日本のジオイド2011 v.2.2 (`gsigeo2011_ver2_2.asc`) を元にしたジオイドデータが同梱されています（測量法に基づく国土地理院長承認（使用）R 5JHs 560）。

License: MIT

本ライブラリは、国土地理院が提供するものではありません。

## Python

### Installation

```
pip install japan-geoid -U
```

### Usage

```python
from japan_geoid import load_embedded_gsigeo2011

# Load the embedded GSIGEO2011 model.
geoid = load_embedded_gsigeo2011()

# Calculate the geoid height.
(lng, lat) = (138.2839817085188, 37.12378643088312)
height = geoid.get_height(lng, lat)
print(f"{lng=} {lat=} {height=}")

# Returns NaN if the input is outside the domain.
geoid.get_height(10.0, 10.0)) # => nan

# The library also works with Numpy.
import numpy as np
geoid.get_heights(
    np.array([138.2839817085188, 141.36199967724426]),
    np.array([37.12378643088312, 43.06539278249951]),
)
```

## Rust

### Installation

```
cargo add japan-geoid
```

### Usage

```rust
use japan_geoid::gsi::load_embedded_gsigeo2011;
use japan_geoid::Geoid;

fn main() {
    // Load the embedded GSIGEO2011 model.
    let geoid = load_embedded_gsigeo2011();

    // Calculate the geoid height.
    let (lng, lat) = (138.2839817085188, 37.12378643088312);
    let height = geoid.get_height(lng, lat);
    println!("Input: (lng: {lng}, lat: {lat}) -> Geoid height: {height}");

    // Returns NaN if the input is outside the domain.
    assert!(f64::is_nan(geoid.get_height(10.0, 10.0)))
}
```

## JavaScript (Wasm)

### Installation

```
npm add japan-geoid
```

### Usage

```javascript
import init, { loadEmbeddedGSIGEO2011 } from "japan-geoid";

await init(); // load .wasm

const geoid = loadEmbeddedGSIGEO2011();

console.log(
  geoid.getHeight(138.2839817085188, 37.12378643088312)
); // => 39.47387115961899

console.log(
  geoid.getHeights(
    [138.2839817085188, 141.36199967724426],
    [37.12378643088312, 43.06539278249951]
  )
); // => Float64Array(2) [ 39.47387115961899, 31.90071200378531 ]
```

## License

MIT License

測量法に基づく国土地理院長承認（使用）R 5JHs 560

## Authors

- Taku Fukada
- [MIERUNE Inc.](https://www.mierune.co.jp/)
