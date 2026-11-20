"""A round post on a Bézier ridge, its side drafted. The block under a cubic Bézier arc, extruded:
in the xz plane the region under the arc with poles (0, 6), (20/3, 14), (40/3, 14), (20, 6) above
z = 0, between x = 0 and x = 20, extruded from y = 0 to y = 20, united with the cylinder of radius
4 about the vertical line through (10, 10), z from 0 to 26; its side drafted by 4°, pull +z,
neutral plane z = 0, into the cone of radius 4 − z·tan 4°."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import face_through
from draft import drafted, united


def build():
    poles = [(0, 6), (20 / 3, 14), (40 / 3, 14), (20, 6)]
    arr = Array1_gp_Pnt(1, 4)
    for i, (x, z) in enumerate(poles):
        arr.SetValue(i + 1, gp_Pnt(x, 0, z))
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(arr)).Edge())
    for a, b in [((20, 0, 6), (20, 0, 0)), ((20, 0, 0), (0, 0, 0)), ((0, 0, 0), (0, 0, 6))]:
        wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(*a), gp_Pnt(*b)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    block = BRepPrimAPI_MakePrism(face, gp_Vec(0, 20, 0)).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, 10, 0), gp_Dir(0, 0, 1)), 4, 26).Shape()
    part = united(block, post)
    side = face_through(part, (14, 10, 24))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
