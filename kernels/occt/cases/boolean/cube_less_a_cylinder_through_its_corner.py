"""A cube less a cylinder standing on its corner, through its bottom. The cube from the origin to (10, 10, 10), less the cylinder of radius 4 about the z axis from z = -3 to z = 6."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    tool = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -3), gp_Dir(0, 0, 1)), 4, 9).Shape()
    cut = BRepAlgoAPI_Cut(cube, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut failed")
    return cut.Shape()
