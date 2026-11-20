"""A cube cut through by a square prism as tall as itself. A cube from (0, 0, 0) to (10, 10, 10), and a block from (3, 3, 0) to (7, 7, 10): 4 by 4 and exactly as tall as the cube, standing on the xy plane inside the cube's footprint, so that its top and its bottom lie in the planes of the cube's top and bottom. Cut the block away from the cube."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(3, 3, 0), gp_Pnt(7, 7, 10)).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
