"""A cube and a slab along its diagonal whose far side reaches a corner, their common part. A cube from (0, 0, 0) to (10, 10, 10) and a slab from z = 0 to z = 10 whose footprint is the rectangle with corners (0, 0), (10, 10), (15, 5) and (5, −5): it is 10√2 long, as long as the cube's diagonal, and 5√2 wide, one long side along the diagonal of the cube's bottom from the corner (0, 0) to the corner (10, 10), its ends square to the diagonal and through those corners. Make the part common to both."""
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
    first = box((0, 0, 0), (10, 10, 10))
    second = prism([(0, 0), (10, 10), (15, 5), (5, -5)], 0, 10)
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two solids")
    return common.Shape()
