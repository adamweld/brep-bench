"""A cube with a corner notched flush with its base. A cube from (0, 0, 0) to (10, 10, 10), and a block from (5, 5, 0) to (15, 15, 5), half the cube's height, standing on the xy plane with its bottom in the plane of the cube's bottom and overlapping the cube's footprint from 5 to 10 in x and y. Cut the block away from the cube."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(5, 5, 0), gp_Pnt(15, 15, 5)).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
