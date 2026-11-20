"""A teardrop prism. Its profile in the xy plane is bounded by one cubic Bézier curve that starts and ends at the origin, with control points (0, 0), (20, 20), (20, −20) and (0, 0), B(t) = (60t(1-t),\ 60t(1-t)(1-2t)), which runs once round from the origin out to (15, 0) and back, clockwise. The loop is closed with a corner at the origin, where the curve leaves along (1, 1) and arrives along (−1, 1), a right angle between the two ends. Extrude the profile straight up the z axis from z = 0 to z = 10.

Round the one vertical edge, above the corner, with a fillet of radius 1: a blend between the two ends of the Bézier wall that rolls a ball of radius 1 between them, replacing the sharp edge with a surface tangent to both and ending flat on the top and base.
"""
from OCP.BRep import BRep_Tool
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

RADIUS = 1
HEIGHT = 10
TOLERANCE = 1e-6


def teardrop_prism():
    points = Array1_gp_Pnt(1, 4)
    for i, (x, y) in enumerate(((0, 0), (20, 20), (20, -20), (0, 0)), start=1):
        points.SetValue(i, gp_Pnt(x, y, 0))
    loop = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(points)).Edge()
    face = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(loop).Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, HEIGHT)).Shape()


def edges_where(shape, test):
    found = []
    explorer = TopExp_Explorer(shape, TopAbs_EDGE)
    while explorer.More():
        edge = TopoDS.Edge(explorer.Current())
        first = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
        last = BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge))
        if test(first, last) and not any(edge.IsSame(e) for e in found):
            found.append(edge)
        explorer.Next()
    return found


def round_edges(shape, edges):
    fillet = BRepFilletAPI_MakeFillet(shape)
    for edge in edges:
        fillet.Add(RADIUS, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the edges")
    return fillet.Shape()


def vertical_corner_edge(shape):
    # The vertical edge above the origin: its ends over (0, 0), at the base and at the top.
    over_origin = lambda a, b: max(abs(a.X()), abs(a.Y()), abs(b.X()), abs(b.Y())) < TOLERANCE
    vertical = [e for e in edges_where(shape, over_origin)
                if abs(BRep_Tool.Pnt_s(TopExp.FirstVertex_s(e)).Z()
                       - BRep_Tool.Pnt_s(TopExp.LastVertex_s(e)).Z()) > 1]
    assert len(vertical) == 1, f"{len(vertical)} vertical edges at the corner"
    return vertical


def build():
    prism = teardrop_prism()
    return round_edges(prism, vertical_corner_edge(prism))
