"""A dumbbell plate shrunk inward by 1.5, past half its neck width, leaving two separate bodies.
The outline is two 10 × 10 squares joined by a 4 × 2 neck, extruded 6 along z."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from offset import offset

OUTLINE = [(0, 0), (10, 0), (10, 4), (14, 4), (14, 0), (24, 0), (24, 10), (14, 10),
           (14, 6), (10, 6), (10, 10), (0, 10)]


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in OUTLINE:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    solid = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 6)).Shape()
    return offset(solid, -1.5)
