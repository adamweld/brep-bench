"""A cube less a cone standing on its corner, through its bottom. The cube from the origin to (10, 10, 10), less the cone about the z axis with its base the disc of radius 5 at z = -2 and its apex at (0, 0, 8)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    tool = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -2), gp_Dir(0, 0, 1)), 5, 0, 10).Shape()
    cut = BRepAlgoAPI_Cut(cube, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut failed")
    return cut.Shape()
