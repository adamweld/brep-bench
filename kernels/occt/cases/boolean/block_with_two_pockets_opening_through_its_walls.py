"""A block with two overlapping pockets of one depth, each breaking out through a side wall. A block
from (0, 0, 0) to (20, 12, 8), with two flat-bottomed pockets cut from its top, each 4 deep, so
that both floors lie in the plane z = 4. The first is the box from (−2, 2, 4) to (8, 10, 9), which
runs out through the x = 0 side wall; the second, partly overlapping it, is the box from (6, 4, 4)
to (22, 8, 9), which runs out through the x = 20 side wall. Cut the first, then the second, from
the block."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    shape = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 12, 8)).Shape()
    for lo, hi in (((-2, 2, 4), (8, 10, 9)), ((6, 4, 4), (22, 8, 9))):
        cut = BRepAlgoAPI_Cut(shape, BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape())
        if not cut.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined a pocket")
        shape = cut.Shape()
    return shape
