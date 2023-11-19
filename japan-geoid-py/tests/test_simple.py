import gzip

import japan_geoid
import numpy as np
from pytest import approx


def test_test():
    with gzip.open("../../gsigeo2011_ver2_2.bin.gz", "rb") as f:
        geoid = japan_geoid.GsiGeoid(f.read())

    assert geoid.get_height(138.2839817085188, 37.12378643088312) == approx(
        39.47387210863509, 1e-12
    )
    assert geoid.get_height(141.36199967724426, 43.06539278249951) == approx(
        31.900711649958033, 1e-12
    )

    print(
        geoid.get_heights(
            np.array([138.2839817085188, 141.36199967724426]),
            np.array([37.12378643088312, 43.06539278249951]),
        )
    )


if __name__ == "__main__":
    test_test()
