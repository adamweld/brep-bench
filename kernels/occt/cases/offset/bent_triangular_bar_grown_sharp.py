"""A bent triangular bar grown by 1, its edges kept sharp. The right triangle (rho, z) = (4, 0),
(6, 0), (6, 2) in the half plane x > 0, turned 90 degrees about the z axis from +x toward +y, grown
by 1 with the moved faces extended until they meet."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp, gp_Pnt

from bench import Refused
from offset import offset


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for rho, z in [(4, 0), (6, 0), (6, 2)]:
        polygon.Add(gp_Pnt(rho, 0, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    bar = BRepPrimAPI_MakeRevol(face, gp.OZ_s(), math.pi / 2).Shape()
    grown = offset(bar, 1, GeomAbs_Intersection)
    if grown.IsNull():
        raise Refused("BRepOffsetAPI_MakeOffsetShape returned no shape for the bent bar")
    return grown
