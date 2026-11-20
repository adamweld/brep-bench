"""Two bars crossed at an angle, united. Both are 2 high on z = 0. The first is the box from
(-6, -1) to (6, 1) in x and y. The second, 8 by 3, from (-4, -1.5) to (4, 1.5), is turned about the
z axis by arctan(3/4)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax1, gp_Pnt, gp_Trsf

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(-6, -1, 0), gp_Pnt(6, 1, 2)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(-4, -1.5, 0), gp_Pnt(4, 1.5, 2)).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp.DZ_s()), math.atan2(3, 4))
    second = BRepBuilderAPI_Transform(second, turn, True).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two bars")
    return fuse.Shape()
