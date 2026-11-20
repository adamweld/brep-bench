"""A cube's two adjacent faces moved, one out and one in. The cube from the origin to (10, 10, 10).

Then edit it directly: move its face at x = 10 out by 3 along +x, to the plane x = 13, and its face
at y = 10, which is adjacent to it, in by 2 along −y, to the plane y = 8. The four faces beside each
follow: they are extended or trimmed to meet the moved ones.

The result is the box from the origin to (13, 8, 10)."""
import math

from bench import face_through
from edit import box, moved

def build():
    cube = box((0, 0, 0), (10, 10, 10))
    right = face_through(cube, (10, 5, 5))
    back = face_through(cube, (5, 10, 5))
    return moved(cube, [(right, 3), (back, -2)])
