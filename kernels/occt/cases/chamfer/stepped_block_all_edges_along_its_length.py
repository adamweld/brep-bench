"""A stepped block with all eight of its edges along its length chamfered. The base from the origin
to (40, 30, 10) and the upper block from (10, 0, 10) to (25, 30, 25), joined, their coplanar faces
merged; every edge along y chamfered by 3."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain

from bench import Refused
from blend import edges_where
from edit import box, united


def stepped_block():
    joined = united(box((0, 0, 0), (40, 30, 10)), box((10, 0, 10), (25, 30, 25)))
    unify = ShapeUpgrade_UnifySameDomain(joined, True, True, True)
    unify.Build()
    return unify.Shape()


def build():
    block = stepped_block()
    # the edges along y: their middle is at y = 15
    edges = edges_where(block, lambda p: abs(p.Y() - 15) < 1e-9)
    assert len(edges) == 8, len(edges)
    chamfer = BRepFilletAPI_MakeChamfer(block)
    for edge in edges:
        chamfer.Add(3, edge)
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the eight edges")
    return chamfer.Shape()
