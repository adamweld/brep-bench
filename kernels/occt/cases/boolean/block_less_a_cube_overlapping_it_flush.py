"""A block less a cube it overlaps, their side faces flush. A cube from (0, 0, 0) to (10, 10, 10), and a block from (5, 0, 0) to (15, 10, 10). Cut the cube away from the block."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(5, 0, 0), gp_Pnt(15, 10, 10)).Shape()
    cut = BRepAlgoAPI_Cut(block, cube)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the block and the cube")
    return cut.Shape()
