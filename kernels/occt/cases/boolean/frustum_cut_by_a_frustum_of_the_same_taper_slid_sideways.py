"""A cone frustum on the z axis (radius 6 at z = 0, radius 3 at z = 8), less a frustum of the same taper on
the line through (4, 0) (radius 5 at z = 0, radius 2 at z = 8)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

Z = gp_Dir(0, 0, 1)


def build():
    first = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), Z), 6, 3, 8).Shape()
    second = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(4, 0, 0), Z), 5, 2, 8).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()
