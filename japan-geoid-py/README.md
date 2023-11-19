# japan-geoid-py

Python library to calculate geoid heights in Japan using [GSI's geoid model](https://fgd.gsi.go.jp/download/geoid.php).

[国土地理院のジオイドモデル](https://fgd.gsi.go.jp/download/geoid.php)を用いて日本のジオイド高を計算する Python 用ライブラリ。

License: MIT License

## Installation

```
pip install japan-geoid -U
```

## Usage

```python
from japan_geoid import GsiGeoid

# オリジナルのASCII形式のジオイドモデルを読み込む。
with open("gsigeo2011_ver2_2.asc", "r") as f:
    geoid = GsiGeoid.from_ascii(f.read())

# ジオイド高を取得する。
(lng, lat) = (138.2839817085188, 37.12378643088312)
height = geoid.get_height(lng, lat)
print(f"{lng=} {lat=} {height=}")

# ジオイドモデルを、効率的なバイナリ形式で保存する。
# この例では更に gzip 形式での圧縮もしている。
# 今後は、ASCII形式のデータを読まずに、このバイナリファイルを利用できる。
import gzip
with gzip.open("gsigeo2011_ver2_2.bin.gz", "wb") as dest_f:
    dest_f.write(geoid.to_binary())

# バイナリ形式のジオイドモデルを読み込む。
with gzip.open("gsigeo2011_ver2_2.bin.gz", "rb") as f:
    geoid = GsiGeoid.from_binary(f.read())

# ジオイド高を取得する。
(lng, lat) = (138.2839817085188, 37.12378643088312)
height = geoid.get_height(lng, lat)
print(f"{lng=} {lat=} {height=}")
```

### Numpy

Use `geoid.get_heights(ndarray_of_lng, ndarray_of_lat)`.
