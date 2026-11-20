"""Two overlapping blocks, united, less a cylinder. The blocks from (0, 0, 0) to (10, 10, 10) and
from (-8, -8, 0) to (8, 3, 10), united, less the cylinder of radius 6 about the z axis from z = 0
to z = 10."""
from bench import Refused
from edit import box, cylinder, less, united


def build():
    blocks = united(box((0, 0, 0), (10, 10, 10)), box((-8, -8, 0), (8, 3, 10)))
    return less(blocks, cylinder((0, 0, 0), 6, 10))
