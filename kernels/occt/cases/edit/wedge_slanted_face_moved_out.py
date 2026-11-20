"""A wedge's slanted face moved out. A wedge: the polygon in the plane z = 0 with corners (0, 0), (10, 0), (4, 8) and (0, 8), extruded
along +z by 6. Its six faces are the bottom, y = 0, 10 by 6; the top, y = 8, 4 by 6; the left side,
x = 0, 8 by 6; the two ends, z = 0 and z = 6; and the slanted face from (10, 0) to (4, 8), in the
plane 4x + 3y = 40, whose outward normal is (4, 3, 0)/5.

Then edit it directly: move the slanted face 2 outward along its normal, to the plane 4x + 3y = 50.
The bottom, the top and the two ends are extended to meet it, so the bottom now runs to x = 12.5 and
the top to x = 6.5.

The result is the wedge with the corners (0, 0), (12.5, 0), (6.5, 8) and (0, 8), extruded along +z
by 6."""
import math

from bench import face_through
from edit import extruded, moved

def build():
    wedge = extruded([(0, 0, 0), (10, 0, 0), (4, 8, 0), (0, 8, 0)], (0, 0, 6))
    slanted = face_through(wedge, (7, 4, 3))
    return moved(wedge, [(slanted, 2)])
