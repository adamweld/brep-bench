"""A cube and a frustum centred on a side face, tangent to the faces beside it, their common part. A cube from (-5, 0, 0) to (5, 10, 10), common with a frustum on the vertical line through the origin, radius 5 at z = 0 and 2.5 at z = 10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-5, 0, 0), gp_Pnt(5, 10, 10)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 2.5, 10).Shape()
    result = BRepAlgoAPI_Common(cube, frustum)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()
