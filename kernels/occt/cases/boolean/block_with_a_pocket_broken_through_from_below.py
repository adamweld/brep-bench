"""A block with a top pocket and a cut from below that meets its floor face to face. A block from (0,
0, 0) to (20, 12, 8). Cut from its top the pocket box from (2, 2, 4) to (10, 10, 9), 4 deep; then
cut from below the box from (6, 4, −1) to (16, 8, 4), whose top face lies in the plane of the
pocket's floor, z = 4. They share no volume; their faces meet at z = 4 over the square 6 ≤ x ≤ 10,
4 ≤ y ≤ 8, area 16."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    shape = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 12, 8)).Shape()
    for lo, hi in (((2, 2, 4), (10, 10, 9)), ((6, 4, -1), (16, 8, 4))):
        cut = BRepAlgoAPI_Cut(shape, BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape())
        if not cut.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined a cut")
        shape = cut.Shape()
    return shape
