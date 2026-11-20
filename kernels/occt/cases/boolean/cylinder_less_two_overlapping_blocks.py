"""A cylinder less two overlapping blocks. A cylinder of radius 6 about the z axis, from z = 0 to
z = 10, less the block from (0, 0, 0) to (10, 10, 10) and the block from (-8, -8, 0) to (8, 3, 10),
which overlap each other: cut by one, then by the other."""
from edit import box, cylinder, less


def build():
    cut = less(cylinder((0, 0, 0), 6, 10), box((0, 0, 0), (10, 10, 10)))
    return less(cut, box((-8, -8, 0), (8, 3, 10)))
