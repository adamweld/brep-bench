"""The outer half of a horn torus shrunk by a half. The part with rho >= 2 of the torus about the z axis
whose tube, of radius 2, runs round the circle of radius 2 in the plane z = 0, shrunk by 1/2."""
from offset import offset, revolved

SHRINK = 0.5


def build():
    half = revolved([('to', 2, -2), ('arc', 2, 0, 2, -90, 90), ('to', 2, 2)], "the half of a horn torus")
    return offset(half, -SHRINK)
