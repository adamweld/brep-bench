"""A cylinder and an equal cylinder centred on its wall, their common part. Two cylinders of radius 3 and height 6, axes 3 apart, their common part."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 3, 6).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(3, 0, 0), gp.DZ_s()), 3, 6).Shape()
    result = BRepAlgoAPI_Common(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()
