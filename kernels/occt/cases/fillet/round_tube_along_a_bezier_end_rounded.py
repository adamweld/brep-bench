"""A round tube swept along a cubic Bézier, its far end edge rounded. The Bézier with control
points (0, 0, 0), (10, 0, 0), (20, 0, 10), (20, 0, 20), a disc of radius 3/2 carried along it,
square to it; the circle around the far end rounded to radius 1/2."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipe
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused, other_case
from blend import edges_where, rounded


def build():
    path = other_case("fillet/rectangular-tube-along-a-bezier-one-edge-rounded").path
    circle = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)), 1.5)).Edge()
    face = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(circle).Wire()).Face()
    pipe = BRepOffsetAPI_MakePipe(path(), face)
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipe could not sweep the disc")
    tube = pipe.Shape()
    rim = edges_where(tube, lambda p: abs(p.Z() - 20) < 1e-6)
    assert len(rim) == 1, len(rim)
    return rounded(tube, 0.5, rim, "the far end's rim")
