"""A prism on a closed Bézier curve, shrunk by 1/2. Four cubic Bézier arcs, the first through (5, 0),
(5, 4), (4, 5), (0, 5) and the others that turned 90°, 180° and 270° about z, close a convex curve;
its inside in z = 0 extruded to z = 8, the whole solid shrunk by 1/2."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from offset import offset

ARC = [(5, 0), (5, 4), (4, 5), (0, 5)]
HEIGHT = 8
SHRINK = 0.5


def build():
    wire = BRepBuilderAPI_MakeWire()
    for k in range(4):
        c, s = math.cos(k * math.pi / 2), math.sin(k * math.pi / 2)
        poles = Array1_gp_Pnt(1, 4)
        for i, (x, y) in enumerate(ARC):
            poles.SetValue(i + 1, gp_Pnt(round(c * x - s * y, 12), round(s * x + c * y, 12), 0))
        wire.Add(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, HEIGHT)).Shape()
    return offset(prism, -SHRINK)
