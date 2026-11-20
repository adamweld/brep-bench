"""A cylinder and a cube tilted about a corner below its base, their common part. A cylinder of radius 3 and height 6 on the z axis, common with a cube of edge 4 at (0, 0, -1) turned 30 degrees about the line through it along (1, 1, 0)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt
import math
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.gp import gp_Ax1, gp_Dir, gp_Trsf

from bench import Refused


def build():
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 3, 6).Shape()
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, -1), 4, 4, 4).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, -1), gp_Dir(1, 1, 0)), math.radians(30))
    cube = BRepBuilderAPI_Transform(cube, turn, True).Shape()
    result = BRepAlgoAPI_Common(cylinder, cube)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()
