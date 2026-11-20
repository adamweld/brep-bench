"""A bar with a drafted side and rounded top edges, notched by a rounded pocket. The bar is 20 wide
(x), 30 long (y) and 10 high, its x = 20 side drafted 10° and its two long top edges rounded to 2.
The pocket is the rounded rectangle 8 ≤ x ≤ 30, 10 ≤ y ≤ 20, corner radius 3, cut from the top to
the floor z = 7, so it opens out through the drafted side."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp_Pnt, gp_Vec

from drafted_bar import FLOOR, bar
from bench import Refused

X0, X1, Y0, Y1, R = 8, 30, 10, 20, 3


def build():
    z = FLOOR
    wire = BRepBuilderAPI_MakeWire()

    def line(a, b):
        wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(*a, z), gp_Pnt(*b, z)).Edge())

    def arc(start, mid, end):
        wire.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(gp_Pnt(*start, z), gp_Pnt(*mid, z),
                                                            gp_Pnt(*end, z)).Value()).Edge())

    c = R * (1 - math.sqrt(0.5))
    line((X0 + R, Y0), (X1 - R, Y0))
    arc((X1 - R, Y0), (X1 - c, Y0 + c), (X1, Y0 + R))
    line((X1, Y0 + R), (X1, Y1 - R))
    arc((X1, Y1 - R), (X1 - c, Y1 - c), (X1 - R, Y1))
    line((X1 - R, Y1), (X0 + R, Y1))
    arc((X0 + R, Y1), (X0 + c, Y1 - c), (X0, Y1 - R))
    line((X0, Y1 - R), (X0, Y0 + R))
    arc((X0, Y0 + R), (X0 + c, Y0 + c), (X0 + R, Y0))
    tool = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(wire.Wire()).Face(), gp_Vec(0, 0, 5)).Shape()
    cut = BRepAlgoAPI_Cut(bar(), tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the pocket")
    return cut.Shape()
