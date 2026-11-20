"""A cylinder and a longer, narrower coaxial cylinder, their common part. A cylinder of radius 4 from z = 0 to 6, common with a coaxial cylinder of radius 2 from z = -3 to 6."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    wide = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 4, 6).Shape()
    narrow = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -3), gp.DZ_s()), 2, 9).Shape()
    result = BRepAlgoAPI_Common(wide, narrow)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()
