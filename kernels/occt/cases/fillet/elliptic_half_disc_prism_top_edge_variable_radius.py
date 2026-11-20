"""A prism on half an ellipse with its curved top edge rounded with a growing radius. The part of
the elliptic cylinder x^2/20^2 + y^2/12^2 <= 1 with y >= 0 and 0 <= z <= 10. The half ellipse where
the top meets the wall is rounded with the radius r = 2 - x / 20: 1 at (20, 0, 10), 3 at
(-20, 0, 10)."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.collections import Array1_gp_Pnt2d
from OCP.gp import gp, gp_Ax2, gp_Elips, gp_Pnt, gp_Pnt2d, gp_Vec

from bench import Refused
from blend import edges_where


def build():
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 20, 12)
    arc = BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi).Edge()
    base = BRepBuilderAPI_MakeEdge(gp_Pnt(-20, 0, 0), gp_Pnt(20, 0, 0)).Edge()
    face = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(arc, base).Wire()).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 10)).Shape()
    edge = edges_where(prism, lambda p: abs(p.Z() - 10) < 1e-6 and p.Y() > 1)
    if len(edge) != 1:
        raise Refused(f"{len(edge)} curved top edges")
    edge = edge[0]
    # A linear law in the abscissa along the edge matches r = 2 - x / 20 only when x is linear in
    # it; it is not, so the law is set at stations along the arc: x = 20 cos(t), r = 2 - cos(t),
    # the stations equally spaced in the ellipse's parameter t.
    fillet = BRepFilletAPI_MakeFillet(prism)
    curve = BRepAdaptor_Curve(edge)
    t0, t1 = curve.FirstParameter(), curve.LastParameter()
    stations = 13
    law = Array1_gp_Pnt2d(1, stations)
    for i in range(stations):
        u = i / (stations - 1)
        t = t0 + (t1 - t0) * u
        law.SetValue(i + 1, gp_Pnt2d(u, 2 - curve.Value(t).X() / 20))
    fillet.Add(law, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the edge with its radius law")
    return fillet.Shape()
