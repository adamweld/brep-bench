"""The outer half of a horn torus grown by 1. The part with rho >= 2 of the torus about the z axis whose
tube, of radius 2, runs round the circle of radius 2 in the plane z = 0: the half-disk of radius 2
about (rho, z) = (2, 0) with rho >= 2, turned about the z axis. Grown by 1."""
from offset import offset, revolved

GROWTH = 1


def build():
    half = revolved([('to', 2, -2), ('arc', 2, 0, 2, -90, 90), ('to', 2, 2)], "the half of a horn torus")
    return offset(half, GROWTH)
