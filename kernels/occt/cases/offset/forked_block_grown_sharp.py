"""A forked block grown by 1 with sharp edges. The polygon in the xy plane through (0, 0), (10, 0),
(10, 4), (13, 8), (9, 8), (6, 4), (4, 4), (1, 8), (-3, 8) and (0, 4), extruded from z = 0 to z = 5."""
from OCP.GeomAbs import GeomAbs_Intersection

from edit import extruded
from offset import offset

OUTLINE = [(0, 0), (10, 0), (10, 4), (13, 8), (9, 8), (6, 4), (4, 4), (1, 8), (-3, 8), (0, 4)]


def build():
    block = extruded([(x, y, 0) for x, y in OUTLINE], (0, 0, 5))
    return offset(block, 1, GeomAbs_Intersection)
