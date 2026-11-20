"""A prism with a Bézier wall shrunk by 0.5 with sharp edges. The profile in the xy plane bounded by
the segment (0, 0)-(10, 0), the cubic Bézier curve from (10, 0) to (0, 8) with control points
(10, 0), (10, 5), (5, 8), (0, 8), and the segment (0, 8)-(0, 0), extruded from z = 0 to z = 6."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineCurve
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp_Pnt, gp_Vec

from offset import offset


def build():
    poles = Array1_gp_Pnt(1, 4)
    for i, (x, y) in enumerate([(10, 0), (10, 5), (5, 8), (0, 8)], 1):
        poles.SetValue(i, gp_Pnt(x, y, 0))
    knots = Array1_double(1, 2)
    knots.SetValue(1, 0.0)
    knots.SetValue(2, 1.0)
    mults = Array1_int(1, 2)
    mults.SetValue(1, 4)
    mults.SetValue(2, 4)
    wall = Geom_BSplineCurve(poles, knots, mults, 3)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(10, 0, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(wall).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 8, 0), gp_Pnt(0, 0, 0)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 6)).Shape()
    return offset(prism, -0.5, GeomAbs_Intersection)
