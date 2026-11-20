"""A triangle turned half a turn about an axis beside its plane. The right triangle with corners
(4, 1, 0), (4, 5, 0), (4, 1, 6), turned 180° about the z axis, right-handed."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt


def build():
    wire = BRepBuilderAPI_MakePolygon(
        gp_Pnt(4, 1, 0), gp_Pnt(4, 5, 0), gp_Pnt(4, 1, 6), True).Wire()
    triangle = BRepBuilderAPI_MakeFace(wire).Face()
    return BRepPrimAPI_MakeRevol(triangle, gp.OZ_s(), math.pi).Shape()
