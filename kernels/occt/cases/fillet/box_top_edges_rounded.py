"""A box with the four edges round its top face rounded. A box from the origin to (20, 10, 6). Round
the four edges around its top face z = 6, each with radius 2; the vertical and bottom edges stay sharp."""
from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

TOP = 6
RADIUS = 2
TOLERANCE = 1e-7


def build():
    box = BRepPrimAPI_MakeBox(20, 10, TOP).Shape()
    fillet = BRepFilletAPI_MakeFillet(box)
    edges = TopExp_Explorer(box, TopAbs_EDGE)
    while edges.More():  # meets each edge once from each of its faces; MakeFillet holds it once
        edge = TopoDS.Edge(edges.Current())
        edges.Next()
        first, last = TopExp.FirstVertex_s(edge), TopExp.LastVertex_s(edge)
        if abs(BRep_Tool.Pnt_s(first).Z() - TOP) < TOLERANCE and abs(BRep_Tool.Pnt_s(last).Z() - TOP) < TOLERANCE:
            fillet.Add(RADIUS, edge)
    assert fillet.NbContours() == 4, f"{fillet.NbContours()} edges round the top face"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the top edges")
    return fillet.Shape()
