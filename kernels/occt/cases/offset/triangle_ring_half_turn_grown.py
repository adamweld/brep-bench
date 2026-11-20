"""A triangular ring segment, a half turn of it, grown by 0.5. The triangle in the xz plane with
corners (4, 0, 0), (8, 0, 0) and (6, 0, 3), turned half a turn about the z axis, grown by 0.5
with its edges kept sharp."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp, gp_Pnt

from offset import offset

DISTANCE = 0.5


def build():
    triangle = BRepBuilderAPI_MakePolygon()
    for x, z in [(4, 0), (8, 0), (6, 3)]:
        triangle.Add(gp_Pnt(x, 0, z))
    triangle.Close()
    face = BRepBuilderAPI_MakeFace(triangle.Wire()).Face()
    segment = BRepPrimAPI_MakeRevol(face, gp.OZ_s(), math.pi).Shape()
    return offset(segment, DISTANCE, GeomAbs_Intersection)
