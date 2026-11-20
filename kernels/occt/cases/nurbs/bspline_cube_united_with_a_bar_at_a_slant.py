"""A B-spline cube united with a bar turned off its sides: the bar from z = 0 to z = 10 on the
rectangle (0, 0), (16, 12), (15.1, 13.2), (-0.9, 1.2)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_NurbsConvert)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

FOOTPRINT = [(0, 0), (16, 12), (15.1, 13.2), (-0.9, 1.2)]


def build():
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in FOOTPRINT:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 10)).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, prism)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and the prism")
    fuse.SimplifyResult()
    return fuse.Shape()
