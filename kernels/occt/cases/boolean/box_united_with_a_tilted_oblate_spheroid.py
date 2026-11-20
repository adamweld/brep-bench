"""A block united with an oblate spheroid tilted about its centre in the top face. The block is the box −10 ≤ x, y ≤ 10, −6 ≤ z ≤ 0; the spheroid has semi-axes 4 and 2,
its centre the origin, in the block's top face, its short axis tilted 30° from z about the x axis
toward +y (see tilted_spheroid.py)."""
from tilted_spheroid import boolean


def build():
    return boolean("fuse")
