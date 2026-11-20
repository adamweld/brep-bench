"""A large cube cut by a small cube tilted with its lower edge just under its top face. The cube from (0, 0, 0) to
(20, 20, 20), and a cube of side 4 from (8, 8, 19) to (12, 12, 23) turned 30° about the line through (8, 8, 19)
parallel to x."""
from math import radians

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused


def build():
    big = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 20, 20)).Shape()
    small = BRepPrimAPI_MakeBox(gp_Pnt(8, 8, 19), gp_Pnt(12, 12, 23)).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(8, 8, 19), gp_Dir(1, 0, 0)), radians(30))
    tilted = BRepBuilderAPI_Transform(small, turn, True).Shape()
    cut = BRepAlgoAPI_Cut(big, tilted)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two cubes")
    return cut.Shape()
