"""An oblique circular cylinder with its top rim rounded. A disc of radius 3 at z = 0 slid to
(4, 0, 10); the circle around the top disc rounded to radius 1/2."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Vec

from blend import edges_where, rounded


def build():
    circle = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 3)).Edge()
    face = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(circle).Wire()).Face()
    solid = BRepPrimAPI_MakePrism(face, gp_Vec(4, 0, 10)).Shape()
    rim = edges_where(solid, lambda p: abs(p.Z() - 10) < 1e-7)
    assert len(rim) == 1, len(rim)
    return rounded(solid, 0.5, rim, "the top rim")
