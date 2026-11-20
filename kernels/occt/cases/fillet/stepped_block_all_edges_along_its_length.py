"""A stepped block with all eight of its edges along its length rounded. The base from the origin
to (40, 30, 10) and the upper block from (10, 0, 10) to (25, 30, 25), joined, their coplanar faces
merged; every edge along y rounded to radius 3."""
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain

from blend import edges_where, rounded
from edit import box, united


def build():
    joined = united(box((0, 0, 0), (40, 30, 10)), box((10, 0, 10), (25, 30, 25)))
    unify = ShapeUpgrade_UnifySameDomain(joined, True, True, True)
    unify.Build()
    block = unify.Shape()
    edges = edges_where(block, lambda p: abs(p.Y() - 15) < 1e-9)  # along y: middle at y = 15
    assert len(edges) == 8, len(edges)
    return rounded(block, 3, edges, "the eight edges along y")
