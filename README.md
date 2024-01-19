# japan-geoid

Rust and Python library for calculating geoid heights in Japan using [GSI's geoid model](https://fgd.gsi.go.jp/download/geoid.php).

国土地理院のジオイドモデル「[日本のジオイド2011](https://fgd.gsi.go.jp/download/geoid.php)」を用いて日本のジオイド高を計算する Rust 用および Python 用のライブラリです。国土地理院が提供するC++のサンプルコードに準拠した補間計算を行います。

本ライブラリは、日本のジオイド2011 v.2.2 (gsigeo2011_ver2_2.asc) を元にしたジオイドデータを含んでいます。This library contains a derivative work based on gsigeo2011_ver2_2.asc, created with permission: 「測量法に基づく国土地理院長承認（使用）R 5JHs 560」 

本ライブラリは、国土地理院が提供するものではありません。

License: MIT

## Use in Python

### Installation

```
pip install japan-geoid -U
```

### Usage

```python
from japan_geoid import GsiGeoid

geoid = GsiGeoid.from_embedded_gsigeo2011()

(lng, lat) = (138.2839817085188, 37.12378643088312)
height = geoid.get_height(lng, lat)
print(f"{lng=} {lat=} {height=}")

# Returns NaN if the input is outside the domain.
geoid.get_height(10.0, 10.0)) # => nan
```

## Use in Rust

### Installation

TODO:

### Usage

```rust
use japan_geoid::{Geoid, MemoryGrid};

fn main() {
    // Load the embedded GSIGEO2011 v2.1 model made from the GSIGEO 2011.
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
```

## LICENSE

MIT

## Authors

- Taku Fukada
- [MIERUNE Inc.](https://www.mierune.co.jp/)
