"""The domed button several OCCT cases share: the solid of revolution about the z axis of the
region of the half-plane y = 0, x >= 0 bounded by the axis, the base, the flare and the dome."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfEllipse
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt


def button(turn):
    """The button turned through `turn` radians from the half-plane y = 0, x >= 0."""
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 2), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), 4, 3)
    dome = GC_MakeArcOfEllipse(ellipse, 0, math.pi / 2, True).Value()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(6, 0, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(6, 0, 0), gp_Pnt(4, 0, 2)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(dome).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 5), gp_Pnt(0, 0, 0)).Edge())
    section = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s(), turn).Shape()
