"""A cube less an equal cube standing against one of its faces. The cube from the origin to
(10, 10, 10), less the cube from (10, 0, 0) to (20, 10, 10)."""
from edit import box, less


def build():
    return less(box((0, 0, 0), (10, 10, 10)), box((10, 0, 0), (20, 10, 10)))
