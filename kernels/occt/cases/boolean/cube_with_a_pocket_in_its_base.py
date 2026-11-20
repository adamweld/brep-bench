"""A cube with a pocket cut flush into its base. A cube from (0, 0, 0) to (10, 10, 10), and a block from (3, 3, 0) to (7, 7, 5): 4 by 4 and 5 high, standing on the xy plane inside the cube's footprint, its bottom in the plane of the cube's bottom. Cut the block away from the cube."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(3, 3, 0), gp_Pnt(7, 7, 5)).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
