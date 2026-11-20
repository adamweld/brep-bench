"""A cube less a block that touches it at one point. A cube from (0, 0, 0) to (10, 10, 10), and a block from (10, 10, −10) to (20, 20, 0). The block lies below the plane z = 0 of the cube's base and beside the cube, and the two meet in one point only, the cube's corner (10, 10, 0). Cut the block away from the cube."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(10, 10, -10), gp_Pnt(20, 20, 0)).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
