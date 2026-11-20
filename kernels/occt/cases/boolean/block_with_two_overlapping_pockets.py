"""A block with two overlapping pockets of one depth. The block from (0, 0, 0) to (20, 12, 8); the box from (2, 2, 4) to
(10, 10, 9) cut from it, then the box from (6, 4, 4) to (16, 8, 9) cut from the result."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    shape = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 12, 8)).Shape()
    for lo, hi in (((2, 2, 4), (10, 10, 9)), ((6, 4, 4), (16, 8, 9))):
        pocket = BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape()
        cut = BRepAlgoAPI_Cut(shape, pocket)
        if not cut.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined a pocket")
        shape = cut.Shape()
    return shape
