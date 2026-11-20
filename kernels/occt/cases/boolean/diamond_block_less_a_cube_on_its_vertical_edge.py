"""A block turned to a diamond on a cube's vertical edge, less the cube. A cube from (0, 0, 0) to (10, 10, 10), and a block 10 high standing on z = 0 whose footprint is the diamond with corners (20, 10), (10, 20), (0, 10), (10, 0). Cut the cube away from the block."""
import math
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    # a 10√2-square box turned a quarter of a right angle about its corner (10, 0): x along (1, 1, 0), y along (-1, 1, 0)
    block = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(10, 0, 0), gp.DZ_s(), gp_Dir(1, 1, 0)),
                                10 * math.sqrt(2), 10 * math.sqrt(2), 10).Shape()
    cut = BRepAlgoAPI_Cut(block, cube)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the block and the cube")
    return cut.Shape()
