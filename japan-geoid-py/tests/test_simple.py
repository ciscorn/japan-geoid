import math

import numpy as np
from japan_geoid import GsiGeoid
from pytest import approx


def test_test():
    geoid = GsiGeoid.from_embedded_gsigeo2011()

    assert geoid.get_height(138.2839817085188, 37.12378643088312) == approx(
        39.473870927576634,  # calculated with proj
        1e-7,
    )
    assert geoid.get_height(141.36199967724426, 43.06539278249951) == approx(
        31.900711175124826,  # calculated with proj
        1e-7,
    )
    assert geoid.get_height(141.36199967724426, 43.06539278249951) == approx(
        31.900711175124826,  # calculated with proj
        1e-7,
    )

    assert math.isnan(geoid.get_height(10.0, 10.0))

    assert geoid.get_heights(
        np.array([138.2839817085188, 141.36199967724426]),
        np.array([37.12378643088312, 43.06539278249951]),
    ) == approx(
        np.array(
            [
                39.473870927576634,  # calculated with proj
                31.900711175124826,  # calculated with proj
            ]
        ),
        1e-7,
    )


if __name__ == "__main__":
    test_test()
