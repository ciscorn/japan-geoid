"""
$ python3 convert_to_bin.py ./gsigeo2011_ver2_2.asc
"""

import gzip
import sys
from pathlib import Path

from japan_geoid import GsiGeoid

if __name__ == "__main__":
    path = Path(sys.argv[1])

    # オリジナルのASCII形式のジオイドモデルを読み込む
    with open(path, "r") as f:
        geoid = GsiGeoid.from_ascii(f.read())

    # ジオイド高を取得する
    (lng, lat) = (138.2839817085188, 37.12378643088312)
    height = geoid.get_height(lng, lat)
    print(f"{lng=} {lat=} {height=}")

    # ジオイドモデルをバイナリ形式で保存する
    with gzip.open(path.with_suffix(".bin.gz"), "wb") as dest_f:
        dest_f.write(geoid.to_binary())

    # バイナリ形式のジオイドモデルを読み込む
    with gzip.open(path.with_suffix(".bin.gz"), "rb") as f:
        geoid = GsiGeoid.from_binary(f.read())

    # ジオイド高を取得する
    (lng, lat) = (138.2839817085188, 37.12378643088312)
    height = geoid.get_height(lng, lat)
    print(f"{lng=} {lat=} {height=}")
