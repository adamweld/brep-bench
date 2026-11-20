"""A prism with a Bézier side, the edge along it rounded. The plane region bounded by the segment
(0, 0)-(12, 0), the cubic Bézier through control points (12, 0), (16, 3), (14, 9), (6, 12), and the
segment (6, 12)-(0, 0), extruded from z = 0 to z = 6. Round the edge where the top meets the curved
side to radius 1."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire, BRepBuilderAPI_MakeFace
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused
from chamfer import edge_through


def build():
    poles = Array1_gp_Pnt(1, 4)
    for i, p in enumerate([(12, 0, 0), (16, 3, 0), (14, 9, 0), (6, 12, 0)], 1):
        poles.SetValue(i, gp_Pnt(*p))
    curve = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(12, 0, 0)).Edge())
    wire.Add(curve)
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(6, 12, 0), gp_Pnt(0, 0, 0)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 6)).Shape()
    # B(1/2) = (13.375, 5.0625, 6) lies on the curve at the top
    top_curve = edge_through(prism, (0.125 * 12 + 0.375 * 16 + 0.375 * 14 + 0.125 * 6,
                                     0.375 * 3 + 0.375 * 9 + 0.125 * 12, 6))
    fillet = BRepFilletAPI_MakeFillet(prism)
    fillet.Add(1, top_curve)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the Bézier edge")
    return fillet.Shape()
