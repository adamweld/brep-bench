"""A spool turned about a tilted axis beside its profile. The eight-sided profile (10, 0),
(14, 0), (14, 3), (12, 3), (12, 9), (14, 9), (14, 12), (10, 12) in the xz plane, turned a whole
turn about the line through the origin in the direction (7, 0, 24)."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt

PROFILE = [(10, 0), (14, 0), (14, 3), (12, 3), (12, 9), (14, 9), (14, 12), (10, 12)]


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, z in PROFILE:
        polygon.Add(gp_Pnt(x, 0, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    axis = gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(7, 0, 24))
    return BRepPrimAPI_MakeRevol(face, axis, 2 * math.pi).Shape()
