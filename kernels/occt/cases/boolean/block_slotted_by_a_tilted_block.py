"""A block slotted by a tilted block. The block from the origin to (12, 8, 6), less a block 10
long, 12 wide (along y) and 4 thick, centred at (6, 4, 6) and turned about the y axis so that its
long edges rise toward +x and +z in the ratio 3 to 4."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(12, 8, 6)).Shape()
    cutter = BRepPrimAPI_MakeBox(gp_Pnt(-5, -6, -2), gp_Pnt(5, 6, 2)).Shape()
    # A right-handed turn about +y takes z toward x, so a rise toward +z is a negative angle.
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 1, 0)), -math.atan2(3, 4))
    cutter = BRepBuilderAPI_Transform(cutter, turn, True).Shape()
    move = gp_Trsf()
    move.SetTranslation(gp_Vec(6, 4, 6))
    cutter = BRepBuilderAPI_Transform(cutter, move, True).Shape()
    cut = BRepAlgoAPI_Cut(block, cutter)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the tilted block")
    return cut.Shape()
