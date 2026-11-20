"""A cube less a block against its face. A cube from (0, 0, 0) to (10, 10, 10), and a block from (10, 5, 0) to (20, 15, 10). The block stands beside the cube on the xy plane, as tall as the cube, and touches the cube only along part of its face x = 10: the block's face x = 10 covers the cube's face from y = 5 to y = 10 and z = 0 to z = 10. Cut the block away from the cube."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(10, 5, 0), gp_Pnt(20, 15, 10)).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
