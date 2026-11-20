"""A bar turned off the sides of a B-spline cube, less the cube. The bar from z = 0 to z = 10, 20 long and 1.5 wide, from the
corner at the origin along (4, 3)/5, less the B-spline cube from (0, 0, 0) to (10, 10, 10)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon, BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused


def prism(points, z0, z1):
    """A prism on the polygon `points` (x, y), from z = z0 to z = z1."""
    wire = BRepBuilderAPI_MakePolygon()
    for x, y in points:
        wire.Add(gp_Pnt(x, y, z0))
    wire.Close()
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, z1 - z0)).Shape()


def build():
    bar = prism([(0, 0), (16, 12), (15.1, 13.2), (-0.9, 1.2)], 0, 10)
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    cut = BRepAlgoAPI_Cut(bar, cube)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bar and the cube")
    return cut.Shape()
