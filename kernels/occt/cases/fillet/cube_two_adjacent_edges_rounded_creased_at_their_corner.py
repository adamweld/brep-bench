"""A cube with two adjacent edges rounded, their rounds creased where they meet. A cube of side 10
between the origin and (10, 10, 10). Round the edge along y where x = 10 meets z = 10, then the
edge along x where y = 0 meets z = 10, each to radius 2, each round running the whole length of its
edge so that they meet in a crease at the corner (10, 0, 10)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from blend import edges_where, rounded

SIDE = 10
RADIUS = 2
TOLERANCE = 1e-7


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    # One round after another, each its own fillet ending on the faces at its ends.
    along_y = edges_where(cube, lambda p: abs(p.X() - SIDE) < TOLERANCE and abs(p.Z() - SIDE) < TOLERANCE)
    once = rounded(cube, RADIUS, along_y, "the edge along y")
    # The second edge, along x at y = 0, z = 10, now ends on the first round's cylinder.
    along_x = edges_where(once, lambda p: abs(p.Y()) < TOLERANCE and abs(p.Z() - SIDE) < TOLERANCE)
    return rounded(once, RADIUS, along_x, "the edge along x")
