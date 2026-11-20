"""A sector of a conical shell: the parallelogram with corners (5, 0), (8, 0), (4, 8), (1, 8) in
the xz plane, turned 240 degrees about the z axis."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax1, gp_Pnt

from bench import Refused


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, z in [(5, 0), (8, 0), (4, 8), (1, 8)]:
        polygon.Add(gp_Pnt(x, 0, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    revol = BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 0), gp.DZ_s()), math.radians(240))
    if not revol.IsDone():
        raise Refused("BRepPrimAPI_MakeRevol declined the section")
    return revol.Shape()
