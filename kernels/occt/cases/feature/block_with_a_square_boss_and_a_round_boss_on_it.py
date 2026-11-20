"""A block with a square boss and a round boss stacked on it: the block 0 ≤ x, y ≤ 40, 0 ≤ z ≤ 10;
a square boss 6 ≤ x, y ≤ 26 from z = 10 to 16 on its top; a round boss of radius 5 on the axis
x = y = 16 from z = 16 to 22 on the square one's top; all joined."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(40, 40, 10)).Shape()
    square = BRepPrimAPI_MakeBox(gp_Pnt(6, 6, 10), gp_Pnt(26, 26, 16)).Shape()
    round_boss = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(16, 16, 16), gp.DZ_s()), 5, 6).Shape()
    first = BRepAlgoAPI_Fuse(block, square)
    if not first.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the square boss")
    second = BRepAlgoAPI_Fuse(first.Shape(), round_boss)
    if not second.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the round boss")
    return second.Shape()
