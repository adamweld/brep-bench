"""A cube's corner notched by a block raised above it. The cube from (0, 0, 0) to (10, 10, 10), the block from (5, 5, 5) to (15, 15, 15), cut away."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(5, 5, 5), gp_Pnt(15, 15, 15)).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
