"""A ring seated in a tube's end, their common part. The tube between radius 6 and radius 10 about
the z axis, from z = 0 to z = 10, made by turning its rectangular section about the axis, and the
torus about the z axis whose middle circle, of radius 8, lies in the plane z = 10, its tube of
radius 2: the torus touches the tube's bore and its outer wall along the tube's two top rims."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    section = BRepBuilderAPI_MakePolygon(gp_Pnt(6, 0, 0), gp_Pnt(10, 0, 0), gp_Pnt(10, 0, 10),
                                         gp_Pnt(6, 0, 10), True).Wire()
    tube = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(section).Face(), gp.OZ_s(),
                                 2 * math.pi).Shape()
    ring = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 10), gp.DZ_s()), 8, 2).Shape()
    common = BRepAlgoAPI_Common(tube, ring)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the tube and the ring")
    return common.Shape()
