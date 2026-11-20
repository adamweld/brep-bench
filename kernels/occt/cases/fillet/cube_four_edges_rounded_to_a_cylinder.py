"""A cube with four edges rounded by half its side. A cube of side 10 between the origin and
(10, 10, 10). Round its four edges parallel to the z axis, each to radius 5."""
from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

SIDE = 10
RADIUS = 5
TOLERANCE = 1e-7


def parallel_to_z(edge):
    a = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
    b = BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge))
    return abs(a.X() - b.X()) < TOLERANCE and abs(a.Y() - b.Y()) < TOLERANCE


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    fillet = BRepFilletAPI_MakeFillet(cube)
    edges = TopExp_Explorer(cube, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        if parallel_to_z(edge):
            fillet.Add(RADIUS, edge)
        edges.Next()
    assert fillet.NbContours() == 4, f"{fillet.NbContours()} edges parallel to z"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the four edges")
    return fillet.Shape()
