"""A cube and a frustum standing through it and reaching below it, their common part. A cube from (-5, -5, 0) to (5, 5, 10), common with a frustum from z = -5 (radius 3) to z = 10 (radius 1.5)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-5, -5, 0), gp_Pnt(5, 5, 10)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -5), gp.DZ_s()), 3, 1.5, 15).Shape()
    result = BRepAlgoAPI_Common(cube, frustum)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()
