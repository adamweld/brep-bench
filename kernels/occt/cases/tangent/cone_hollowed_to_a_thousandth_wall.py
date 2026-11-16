"""A cone, its base of radius 5 at z = 0 and its apex 10 above it on the z axis, less a cone of base
radius 4.999 with the same base plane and the same apex. The first less the second."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

OUTER = 5.0
INNER = 4.999
HEIGHT = 10.0


def build():
    outer = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), OUTER, 0, HEIGHT).Shape()
    inner = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), INNER, 0, HEIGHT).Shape()
    result = BRepAlgoAPI_Cut(outer, inner)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()
