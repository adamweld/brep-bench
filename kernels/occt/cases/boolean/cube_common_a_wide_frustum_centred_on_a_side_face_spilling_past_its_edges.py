"""A cube and a wide frustum centred on a side face, spilling past its edges, their common part. A cube from (-5, 0, 0) to (5, 10, 10), common with a frustum on the vertical line through the origin, radius 7 at z = 0 and 3 at z = 10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-5, 0, 0), gp_Pnt(5, 10, 10)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 7, 3, 10).Shape()
    result = BRepAlgoAPI_Common(cube, frustum)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()
