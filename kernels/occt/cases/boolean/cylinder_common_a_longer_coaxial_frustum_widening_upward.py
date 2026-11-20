"""A cylinder and a longer coaxial frustum, their common part. A cylinder of radius 4 from z = 0 to 10, common with a coaxial frustum from z = -5 (radius 1.5) to z = 10 (radius 3)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 4, 10).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -5), gp.DZ_s()), 1.5, 3, 15).Shape()
    result = BRepAlgoAPI_Common(cylinder, frustum)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()
