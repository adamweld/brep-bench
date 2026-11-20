"""A cylinder of radius 3 and height 6, less a cube of edge 4 turned 30 degrees about its face diagonal, with a corner at (0, 0, −1)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused


def build():
    cylinder = BRepPrimAPI_MakeCylinder(3, 6).Shape()
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(4, 4, 4)).Shape()
    # Turn the cube 30 degrees about the diagonal (1, 1, 0) through its corner at the origin,
    # then lower it by 1 so that the corner sits at (0, 0, -1).
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(1, 1, 0)), math.radians(30))
    cube = BRepBuilderAPI_Transform(cube, turn).Shape()
    lower = gp_Trsf()
    lower.SetTranslation(gp_Vec(0, 0, -1))
    cube = BRepBuilderAPI_Transform(cube, lower).Shape()
    return _cut(cylinder, cube)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()
