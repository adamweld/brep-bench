"""A cube less a ball centred on its corner. The cube from the origin to (10, 10, 10), less the ball of radius 4 centred on the origin."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    tool = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 4).Shape()
    cut = BRepAlgoAPI_Cut(cube, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut failed")
    return cut.Shape()
