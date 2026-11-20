"""A block and an identical block, their common part. Two blocks, each from (0, 0, 0) to (10, 8, 6), one exactly on the other. Make the part common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon, BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused


def box(lo, hi):
    return BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape()


def prism(points, z0, z1):
    """A prism on the polygon `points` (x, y), from z = z0 to z = z1."""
    wire = BRepBuilderAPI_MakePolygon()
    for x, y in points:
        wire.Add(gp_Pnt(x, y, z0))
    wire.Close()
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, z1 - z0)).Shape()


def build():
    first = box((0, 0, 0), (10, 8, 6))
    second = box((0, 0, 0), (10, 8, 6))
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two solids")
    return common.Shape()
