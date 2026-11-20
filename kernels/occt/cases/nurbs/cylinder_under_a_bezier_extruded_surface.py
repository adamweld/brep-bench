"""A cylinder cut away above an extruded Bezier curve. A cylinder of radius 10, z from 0 to 20, kept below
the surface z = g(x), g the cubic Bezier through (-12,6), (-4,14), (4,2), (12,10), extruded along y."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def box(x0, y0, z0, x1, y1, z1):
    return BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, z0), gp_Pnt(x1, y1, z1)).Shape()


def run(op, name):
    if not op.IsDone():
        raise Refused(f"{name} declined")
    return op.Shape()

import math


from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Vec


def build():
    poles = Array1_gp_Pnt(1, 4)
    for i, (x, z) in enumerate([(-12, 6), (-4, 14), (4, 2), (12, 10)]):
        poles.SetValue(i + 1, gp_Pnt(x, -15, z))
    curve = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(curve)
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(12, -15, 10), gp_Pnt(12, -15, -5)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(12, -15, -5), gp_Pnt(-12, -15, -5)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(-12, -15, -5), gp_Pnt(-12, -15, 6)).Edge())
    under = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(wire.Wire()).Face(), gp_Vec(0, 30, 0)).Shape()
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 10, 20).Shape()
    return run(BRepAlgoAPI_Common(drum, under), "BRepAlgoAPI_Common")
