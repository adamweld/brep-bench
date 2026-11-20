"""A B-spline cube and a bar turned off its sides, from a corner, their common part. A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a bar from z = 0 to z = 10, its faces planes, whose footprint is the rectangle 20 long and 1.5 wide with corners (0, 0), (16, 12), (15.1, 13.2) and (−0.9, 1.2): from the corner (0, 0) its length runs 20 along (4/5, 3/5) and its width 1.5 along (−3/5, 4/5). The bar starts at the cube's corner at the origin, with its end square to its length and through that corner, and runs out through the cube's face x = 10. Make the solid common to both."""
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
    first = BRepBuilderAPI_NurbsConvert(box((0, 0, 0), (10, 10, 10)), True).Shape()
    second = prism([(0, 0), (16, 12), (15.1, 13.2), (-0.9, 1.2)], 0, 10)
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two solids")
    return common.Shape()
