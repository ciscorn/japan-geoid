"""Pure-Python implementation"""

import gzip
from array import array
from math import nan
from struct import pack, unpack


class GsiGeoid:
    def __init__(self, f):
        self._load_bin(f)

    @staticmethod
    def convert_asc_to_bin(src_f, dest_f):
        # Header
        h0, h1, h2, h3, h4, h5, h6, h7 = src_f.readline().strip().split()
        _lat_min = float(h0)
        _lng_min = float(h1)
        assert h2 == "0.016667"
        assert h3 == "0.025000"
        _lat_denom = 60
        _lng_denom = 40
        _n_lat = int(h4)
        _n_lng = int(h5)
        assert _lat_min == 20
        assert _lng_min == 120
        assert _n_lat == 1801
        assert _n_lng == 1201
        ikind = int(h6)
        assert ikind == 1
        version = h7.strip()
        assert version == "ver2.2"

        dest_f.write(
            pack(
                "<HHHHffH",
                _n_lng,
                _n_lat,
                _lng_denom,
                _lat_denom,
                _lng_min,
                _lat_min,
                ikind,
            )
        )
        dest_f.write(version.encode("ascii") + b"\0" * (10 - len(version)))
        points = array(
            "i",
            (int(v.replace(".", "")) for line in src_f for v in line.split()),
        )
        assert len(points) == _n_lng * _n_lat

        dest_f.write(points.tobytes())

    def _load_bin(self, f):
        (
            self._n_lng,
            self._n_lat,
            self._lng_denom,
            self._lat_denom,
            self._lng_min,
            self._lat_min,
            self._ikind,
        ) = unpack("<HHHHffH", f.read(18))
        assert f.read(10) == b"ver2.2\0\0\0\0"
        points = array("i")
        points.frombytes(f.read())
        self._points = [float(h / 10000) if h != 9990000 else nan for h in points]

    @staticmethod
    def _bilinear(
        x: float, y: float, v00: float, v01: float, v10: float, v11: float
    ) -> float:
        """Bilinear interpolation"""

        if x == 0:
            if y == 0:
                return v00
            else:
                return v00 * (1 - y) + v10 * (1 - x) * y
        elif y == 0:
            return v00 * (1 - x) + v01 * x
        else:
            return (
                v00 * (1 - x) * (1 - y)
                + v01 * x * (1 - y)
                + v10 * (1 - x) * y
                + v11 * x * y
            )

    def get_height(self, lng: float, lat: float) -> float:
        x, x_residual = divmod((lng - self._lng_min) * self._lng_denom, 1)
        y, y_residual = divmod((lat - self._lat_min) * self._lat_denom, 1)
        x = int(x)
        y = int(y)
        if x < 0 or self._n_lng <= x or y < 0 or self._n_lat <= y:
            return nan

        n_lng = self._n_lng
        n_lat = self._n_lat
        return self._bilinear(
            x_residual,
            y_residual,
            self._points[n_lng * y + x],
            self._points[n_lng * y + (x + 1)] if x < n_lng - 1 else nan,
            self._points[n_lng * (y + 1) + x] if y < n_lat - 1 else nan,
            (
                self._points[n_lng * (y + 1) + (x + 1)]
                if y < n_lat - 1 and x < n_lng - 1
                else nan
            ),
        )


if __name__ == "__main__":
    # 初回のみ必要（元データをバイナリに変換）
    with open("./gsigeo2011_ver2_2.asc") as src_f:
        with gzip.open("./gsigeo2011_ver2_2.bin.gz", "wb") as dest_f:
            GsiGeoid.convert_asc_to_bin(src_f, dest_f)

    # 2回目以降はこれだけでいい
    with gzip.open("./gsigeo2011_ver2_2.bin.gz", "rb") as f:
        geoid = GsiGeoid(f)

    z = geoid.get_height(138.2839817085188, 37.12378643088312)
    print(z)
    z = geoid.get_height(141.36199967724426, 43.06539278249951)
    print(z)
