"""A cylinder with a square channel cut across it, off its axis: a cylinder of radius 10 on the z
axis, 0 ≤ z ≤ 14, and the channel 1 ≤ y ≤ 7, 3 ≤ z ≤ 9 along x, from the near wall to the far one."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cylinder = BRepPrimAPI_MakeCylinder(10, 14).Shape()
    # a tool that begins beside the cylinder and runs on past the far wall
    tool = BRepPrimAPI_MakeBox(gp_Pnt(-12, 1, 3), gp_Pnt(12, 7, 9)).Shape()
    cut = BRepAlgoAPI_Cut(cylinder, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the channel")
    return cut.Shape()
