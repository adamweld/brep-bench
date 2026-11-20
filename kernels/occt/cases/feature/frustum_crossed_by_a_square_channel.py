"""A cone frustum with a square channel cut across it, off its axis: radius 12 at z = 0 and 6 at
z = 14 on the z axis, and the channel 1 ≤ y ≤ 7, 3 ≤ z ≤ 9 along x, from the near wall to the far one."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    frustum = BRepPrimAPI_MakeCone(12, 6, 14).Shape()
    tool = BRepPrimAPI_MakeBox(gp_Pnt(-14, 1, 3), gp_Pnt(14, 7, 9)).Shape()
    cut = BRepAlgoAPI_Cut(frustum, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the channel")
    return cut.Shape()
