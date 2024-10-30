"""
usage: isg2asc GSIGEO2024beta.isg
"""

import sys
from pathlib import Path

data_points = []

filename = Path(sys.argv[1])

with open(filename, "r") as f:
    for line in f:
        if line.strip().startswith("end_of_head ========"):
            break

    for line in f:
        data_points.extend(v for v in line.split())

NX = 1601
NY = 2101

assert len(data_points) == NX * NY

with open(filename.with_suffix(".asc"), "w") as f:
    f.write("15.00000 120.00000 0.016667 0.025000 2101 1601 1 ver-beta")
    f.write("\n")
    f.write("\n")
    for i in range(NY):
        for j in range(NX):
            v = data_points[(NY - i - 1) * NX + j]
            if v == "-9999.0000":
                v = "999.0000"
            f.write(v)
            f.write(" ")
        f.write("\n")
