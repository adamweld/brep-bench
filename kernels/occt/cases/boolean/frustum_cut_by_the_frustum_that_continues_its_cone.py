"""A cone frustum on the z axis, z = 0 (radius 6) to z = 10 (radius 4), less the frustum that
continues its cone, z = 10 (radius 4) to z = 20 (radius 2)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

Z = gp_Dir(0, 0, 1)


def build():
    first = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), Z), 6, 4, 10).Shape()
    second = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 10), Z), 4, 2, 10).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()
