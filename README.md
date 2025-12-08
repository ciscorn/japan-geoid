# japan-geoid

[![Test](https://github.com/ciscorn/japan-geoid/actions/workflows/test.yml/badge.svg)](https://github.com/ciscorn/japan-geoid/actions/workflows/test.yml)
[![Maturin](https://github.com/ciscorn/japan-geoid/actions/workflows/maturin.yml/badge.svg)](https://github.com/ciscorn/japan-geoid/actions/workflows/maturin.yml)
[![codecov](https://codecov.io/gh/ciscorn/japan-geoid/graph/badge.svg?token=c9T2ayChfw)](https://codecov.io/gh/ciscorn/japan-geoid)
[![Crates.io Version](https://img.shields.io/crates/v/japan-geoid)](https://crates.io/crates/japan-geoid)
[![PyPI - Version](https://img.shields.io/pypi/v/japan-geoid)](https://pypi.org/project/japan-geoid/)
[![NPM Version](https://img.shields.io/npm/v/japan-geoid)](https://www.npmjs.com/package/japan-geoid)

A library for calculating geoid heights in Japan using GSI's geoid model. It is implemented in Rust and additionally supports Python and JavaScript.

日本のジオイド高を計算するためライブラリです。Rust で実装されており、Python と JavaScript (Wasm) でも利用できます。国土地理院のジオイドモデルを用いて、国土地理院による C++ のサンプルコードに準拠した補間計算を行います。


## Supported Models

| Model | Function | Description |
|-------|----------|-------------|
| GSIGEO2011 | `load_embedded_gsigeo2011` | [日本のジオイド2011](https://www.gsi.go.jp/buturisokuchi/grageo_reference.html) v.2.2 |
| JPGEO2024 | `load_embedded_jpgeo2024` | [ジオイド2024](https://service.gsi.go.jp/kiban/app/geoid/) (JGD2024用・海域含む大容量) |
| Hrefconv2024 | `load_embedded_hrefconv2024` | [基準面補正パラメータ2024](https://service.gsi.go.jp/kiban/app/geoid/) (離島用) |
| **JPGEO2024+Hrefconv2024** | `load_embedded_jpgeo2024_hrefconv2024` | [JPGEO2024+Hrefconv2024](https://service.gsi.go.jp/kiban/app/geoid/)結合版 (陸域のみ・離島対応・小容量) |

測量法に基づく国土地理院長承認（使用）R 5JHs 560

License: MIT

本ライブラリは、国土地理院が提供するものではありません。


## Python

### Installation

```
pip install japan-geoid -U
```

### Usage

```python
from japan_geoid import (
    load_embedded_gsigeo2011,
    load_embedded_jpgeo2024,
    load_embedded_jpgeo2024_hrefconv2024,
)

# Load the embedded geoid model.
geoid = load_embedded_jpgeo2024_hrefconv2024()

# Calculate the geoid height.
(lng, lat) = (138.2839817085188, 37.12378643088312)
height = geoid.get_height(lng, lat)
print(f"{lng=} {lat=} {height=}")

# Returns NaN if the input is outside the domain.
geoid.get_height(10.0, 10.0) # => nan

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
use japan_geoid::gsi::load_embedded_jpgeo2024_hrefconv2024;
use japan_geoid::Geoid;

fn main() {
    // Load the embedded geoid model.
    let geoid = load_embedded_jpgeo2024_hrefconv2024();

    // Calculate the geoid height.
    let (lng, lat) = (138.2839817085188, 37.12378643088312);
    let height = geoid.get_height(lng, lat);
    println!("Input: (lng: {lng}, lat: {lat}) -> Geoid height: {height}");

    // Returns NaN if the input is outside the domain.
    assert!(f64::is_nan(geoid.get_height(10.0, 10.0)))
}
```


## JavaScript (Wasm) - Experimental

### Installation

```
npm install japan-geoid
```

### Usage

```javascript
import init, {
  loadEmbeddedGSIGEO2011,
  loadEmbeddedJPGEO2024,
  loadEmbeddedJPGEO2024Hrefconv2024,
} from "japan-geoid";

await init(); // load .wasm

const geoid = loadEmbeddedJPGEO2024Hrefconv2024();

console.log(
  geoid.getHeight(138.2839817085188, 37.12378643088312)
); // => 39.56593692999162

console.log(
  geoid.getHeights(
    new Float64Array([138.2839817085188, 141.36199967724426]),
    new Float64Array([37.12378643088312, 43.06539278249951])
  )
); // => Float64Array(2) [ 39.56593692999162, 31.98825817633539 ]
```

### Build

```bash
wasm-pack build -t web
```


## License

MIT License


## Authors

- Taku Fukada ([@ciscorn](https://github.com/ciscorn)) - 測量法に基づく国土地理院長承認（使用）R 5JHs 560
- [@rot1204](https://github.com/rot1204)
- and [all contributors][https://github.com/ciscorn/japan-geoid/graphs/contributors]!


