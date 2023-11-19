"""
JGD2011 3D (EPSG:6697) -> WGS 84 Geograhic 3D (EPSG:4979) -> WGS 84 Geocentric 3D (EPSG:4978)

$ echo 34.290 135.630 0 | python3 geog2geoc.py
"""

import gzip
import math
import sys

import pyproj
from japan_geoid import GsiGeoid

# WGS 84 楕円体のパラメータ
A = 6378137.0  # 長半径
INV_F = 298.257223563  # 偏平率の逆数

# 楕円体パラメータから導ける値
_F = 1 / INV_F  # 偏平率
# _B = A * (1 - _F)  # 短半径
_E_SQ = _F * (2 - _F)  # 離心率の2乗
# _E = _E_SQ**0.5  # 離心率


def geog2geoc(lat, lng, ellipsoidal_heght):
    (lat_deg, lng_deg, height) = (lat, lng, ellipsoidal_heght)
    (lat_rad, lng_rad) = (math.radians(lat_deg), math.radians(lng_deg))

    tan_psi = (1 - _E_SQ) * math.tan(lat_rad)
    z = A * (1 / (1 / (tan_psi * tan_psi) + 1 / ((1 - _F) ** 2))) ** 0.5
    r = A * (1 / (1 + (tan_psi * tan_psi) / ((1 - _F) ** 2))) ** 0.5

    x = r * math.cos(lng_rad)
    y = r * math.sin(lng_rad)
    dhz = math.sin(lat_rad)
    dhx = math.cos(lat_rad) * math.cos(lng_rad)
    dhy = math.cos(lat_rad) * math.sin(lng_rad)
    return (x + dhx * height, y + dhy * height, z + dhz * height)


if __name__ == "__main__":
    with gzip.open("../gsigeo2011_ver2_2.bin.gz", "rb") as f:
        geoid = GsiGeoid.from_binary(f.read())

    proj_geog_tr = pyproj.Transformer.from_crs("EPSG:6697", "EPSG:4978")
    proj_geoc_tr = pyproj.Transformer.from_crs("EPSG:6697", "EPSG:4979")

    for line in sys.stdin:
        (lat, lng, elevation) = [float(v) for v in line.strip().split()]

        # PROJ による変換 (JGD2011 -> WGS84 Geographic 3D)
        proj_lat, proj_lng, proj_height = proj_geoc_tr.transform(lat, lng, elevation)
        # PROJ による変換 (JGD2011 -> WGS84 Geocentric 3D)
        proj_x, proj_y, proj_z = proj_geog_tr.transform(lat, lng, elevation)

        # 自前で変換
        geoid_height = geoid.get_height(lng, lat)
        height = geoid_height + elevation  # ellipsoidal height
        x, y, z = geog2geoc(lat, lng, height)

        print(f"{geoid_height=}")
        print(f"{height=} {proj_height=} diff[m]={height - proj_height}")
        print(f"OURS: {x=} {y=} {z=}")
        print(f"PROJ: {proj_x=} {proj_y=} {proj_z=}")
        geoc_diff = ((proj_x - x) ** 2 + (proj_y - y) ** 2 + (proj_z - z) ** 2) ** 0.5
        print(f"distance[m]={geoc_diff}")
