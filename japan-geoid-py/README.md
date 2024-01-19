# japan-geoid-py

Python library to calculate geoid heights in Japan using [GSI's geoid model](https://fgd.gsi.go.jp/download/geoid.php).

国土地理院のジオイドモデル「[日本のジオイド2011](https://fgd.gsi.go.jp/download/geoid.php)」を用いて日本のジオイド高を計算する Rust 用および Python 用のライブラリです。国土地理院が提供するC++のサンプルコードに準拠した補間計算を行います。

本ライブラリは、日本のジオイド2011 v.2.2 (gsigeo2011_ver2_2.asc) を元にしたジオイドデータを含んでいます。This library contains a derivative work based on gsigeo2011_ver2_2.asc, created with permission: 「測量法に基づく国土地理院長承認（使用）R 5JHs 560」 

本ライブラリは、国土地理院が提供するものではありません。

License: MIT

## Installation

```
pip install japan-geoid -U
```

## Usage

```python
from japan_geoid import GsiGeoid

geoid = GsiGeoid.from_embedded_gsigeo2011()

(lng, lat) = (138.2839817085188, 37.12378643088312)
height = geoid.get_height(lng, lat)
print(f"{lng=} {lat=} {height=}")

# Returns NaN if the input is outside the domain.
geoid.get_height(10.0, 10.0)  # => nan
```

### With Numpy

This library also works with Numpy to perform high-performance vectorized calculations.

```python
import numpy as np

geoid.get_heights(
    np.array([138.2839817085188, 141.36199967724426, ...]),
    np.array([37.12378643088312, 43.06539278249951, ...]),
)
```
