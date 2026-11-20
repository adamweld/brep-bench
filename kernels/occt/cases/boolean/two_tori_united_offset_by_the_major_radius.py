"""Two equal tori united, the second centered on the first's core circle. Major radius 10, minor
radius 3, axes parallel to z, centered at the origin and at (10, 0, 0)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 10, 3).Shape()
    second = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(10, 0, 0), gp_Dir(0, 0, 1)), 10, 3).Shape()
    united = BRepAlgoAPI_Fuse(first, second)
    if not united.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not unite the two tori")
    return united.Shape()
