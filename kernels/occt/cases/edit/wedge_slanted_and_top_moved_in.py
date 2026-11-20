"""A wedge's slanted face and top moved in. A wedge: the polygon in the plane z = 0 with corners (0, 0), (10, 0), (4, 8) and (0, 8), extruded
along +z by 6. Its six faces are the bottom, y = 0, 10 by 6; the top, y = 8, 4 by 6; the left side,
x = 0, 8 by 6; the two ends, z = 0 and z = 6; and the slanted face from (10, 0) to (4, 8), in the
plane 4x + 3y = 40, whose outward normal is (4, 3, 0)/5.

Then edit it directly: move two adjacent faces in, the top by 1, along −y, to the plane y = 7, and
the slanted face by 1 along its inward normal, to the plane 4x + 3y = 35. The faces beside them are
trimmed to meet them: the bottom now runs to x = 8.75 and the top to x = 3.5.

The result is the wedge with the corners (0, 0), (8.75, 0), (3.5, 7) and (0, 7), extruded along +z
by 6."""
import math

from bench import face_through
from edit import extruded, moved

def build():
    wedge = extruded([(0, 0, 0), (10, 0, 0), (4, 8, 0), (0, 8, 0)], (0, 0, 6))
    slanted = face_through(wedge, (7, 4, 3))
    top = face_through(wedge, (2, 8, 3))
    return moved(wedge, [(slanted, -1), (top, -1)])
