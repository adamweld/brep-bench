"""A block with a pocket and a second pocket abutting it along a wall. A block from (0, 0, 0) to (20,
12, 8), with two flat-bottomed pockets cut from its top, each 4 deep, floors both in the plane z =
4: the first is the box from (2, 2, 4) to (8, 10, 9), the second the box from (8, 4, 4) to (14, 8,
9). They touch along the square x = 8, 4 ≤ y ≤ 8, 4 ≤ z ≤ 8 and share no volume. Cut the first,
then the second, from the block."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    shape = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 12, 8)).Shape()
    for lo, hi in (((2, 2, 4), (8, 10, 9)), ((8, 4, 4), (14, 8, 9))):
        cut = BRepAlgoAPI_Cut(shape, BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape())
        if not cut.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined a pocket")
        shape = cut.Shape()
    return shape
