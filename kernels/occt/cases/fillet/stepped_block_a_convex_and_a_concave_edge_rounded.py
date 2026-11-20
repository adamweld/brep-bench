"""A stepped block with a convex and a concave edge along its length rounded to different radii.
The base from the origin to (40, 30, 10) and the upper block from (10, 0, 10) to (25, 30, 25),
joined, their coplanar faces merged; the edge through (10, y, 25) rounded to radius 4 and the edge
through (25, y, 10) to radius 5."""
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain

from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet

from bench import Refused
from blend import edges_where
from edit import box, united


def build():
    joined = united(box((0, 0, 0), (40, 30, 10)), box((10, 0, 10), (25, 30, 25)))
    unify = ShapeUpgrade_UnifySameDomain(joined, True, True, True)
    unify.Build()
    block = unify.Shape()
    convex = edges_where(block, lambda p: abs(p.X() - 10) < 1e-9 and abs(p.Z() - 25) < 1e-9)
    concave = edges_where(block, lambda p: abs(p.X() - 25) < 1e-9 and abs(p.Z() - 10) < 1e-9)
    assert len(convex) == 1 and len(concave) == 1


    fillet = BRepFilletAPI_MakeFillet(block)
    fillet.Add(4, convex[0])
    fillet.Add(5, concave[0])
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the two edges")
    return fillet.Shape()
