"""A cube with an edge chamfered and the chamfer's two long edges rounded. A cube of side 10
between the origin and (10, 10, 10). Chamfer the edge where x = 10 and y = 10 meet by a distance
of 2, then round to radius 2 the two long edges of the chamfer face, at (8, 10) and (10, 8)."""
from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer, BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

SIDE = 10
DISTANCE = 2
RADIUS = 2
TOLERANCE = 1e-7


def vertical_edges_at(shape, x, y):
    """The edges of the shape that run parallel to z through the point (x, y)."""
    found = []
    edges = TopExp_Explorer(shape, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        a = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
        b = BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge))
        if all(abs(p.X() - x) < TOLERANCE and abs(p.Y() - y) < TOLERANCE for p in (a, b)):
            found.append(edge)
        edges.Next()
    return found


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    chamfer = BRepFilletAPI_MakeChamfer(cube)
    for edge in vertical_edges_at(cube, SIDE, SIDE)[:1]:
        chamfer.Add(DISTANCE, edge)
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the edge")
    chamfered = chamfer.Shape()
    fillet = BRepFilletAPI_MakeFillet(chamfered)
    for x, y in ((SIDE - DISTANCE, SIDE), (SIDE, SIDE - DISTANCE)):
        for edge in vertical_edges_at(chamfered, x, y)[:1]:
            fillet.Add(RADIUS, edge)
    assert fillet.NbContours() == 2, f"{fillet.NbContours()} edges beside the chamfer"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the chamfer's edges")
    return fillet.Shape()
