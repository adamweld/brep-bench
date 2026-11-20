"""A cylinder notched by a wider, shorter one, one edge of the notch rounded. The tall cylinder of
radius 10 about the z axis, z from 0 to 12, less the cylinder of radius 6 about the vertical line
through (14, 0), z from 0 to 8; the vertical edge of the notch at y > 0 rounded to radius 2."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

PX = 260 / 28
PY = (100 - PX * PX) ** 0.5


def is_the_edge(edge):
    """Whether `edge` is the straight vertical edge standing over (PX, +PY)."""
    curve = BRepAdaptor_Curve(edge)
    p = curve.Value((curve.FirstParameter() + curve.LastParameter()) / 2)
    q = curve.Value(curve.FirstParameter())
    return (abs(p.X() - PX) < 1e-6 and abs(p.Y() - PY) < 1e-6 and abs(q.X() - p.X()) < 1e-6)


def build():
    tall = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 10, 12).Shape()
    wide = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(14, 0, 0), gp.DZ_s()), 6, 8).Shape()
    cut = BRepAlgoAPI_Cut(tall, wide)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the notch")
    notched = cut.Shape()
    fillet = BRepFilletAPI_MakeFillet(notched)
    found = False
    edges = TopExp_Explorer(notched, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        if is_the_edge(edge):
            fillet.Add(2, edge)
            found = True
            break
        edges.Next()
    if not found:
        raise Refused("the notch's edge at y > 0 was not found")
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the edge")
    return fillet.Shape()
