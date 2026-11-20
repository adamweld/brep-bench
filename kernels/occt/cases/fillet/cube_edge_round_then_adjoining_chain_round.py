"""A cube's edge rounded, then the chain of edges beside the round rounded. A cube of side 10
between the origin and (10, 10, 10). Round the edge where its faces x = 10 and z = 10 meet to
radius 3, then, as a second round, round to radius 2 the chain of three tangent edges where the
face y = 0 meets the top, the first round and the face x = 10."""
from blend import edges_where, rounded
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused

SIDE = 10
FIRST, SECOND = 3, 2
TOLERANCE = 1e-7


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    edge = edges_where(cube, lambda p: abs(p.X() - SIDE) < TOLERANCE and abs(p.Z() - SIDE) < TOLERANCE)
    if len(edge) != 1:
        raise Refused(f"{len(edge)} edges at x = z = {SIDE}")
    first = rounded(cube, FIRST, edge, "the cube's edge")

    # One edge of the chain, the line along the top at y = 0: the fillet builder follows the
    # tangent chain from it by itself.
    top = edges_where(first, lambda p: abs(p.Y()) < TOLERANCE and abs(p.Z() - SIDE) < TOLERANCE)
    if len(top) != 1:
        raise Refused(f"{len(top)} edges along the top of the face y = 0")
    return rounded(first, SECOND, top, "the chain beside the first round")
