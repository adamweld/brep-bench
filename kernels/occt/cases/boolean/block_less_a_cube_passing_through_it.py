"""A bar through a cube, less the cube. A cube from (0, 0, 0) to (10, 10, 10), and a bar from (3, −5, 0) to (7, 15, 6). Cut the cube away from the bar."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(3, -5, 0), gp_Pnt(7, 15, 6)).Shape()
    cut = BRepAlgoAPI_Cut(block, cube)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the block and the cube")
    return cut.Shape()
