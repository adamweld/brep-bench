"""A block turned three-four-five across a cube's corner, less the cube. A cube from (0, 0, 0) to (10, 10, 10), and a block 10 by 6 by 10 turned about the upright. One corner of the block is (6, 1, 0), and the block's three edges from that corner run 10 along (4/5, 3/5, 0), 6 along (−3/5, 4/5, 0) and 10 straight up. Cut the cube away from the block."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    # the box's x runs along the placement's x direction, its y along z × x = (−3/5, 4/5, 0)
    block = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(6, 1, 0), gp.DZ_s(), gp_Dir(4, 3, 0)),
                                10, 6, 10).Shape()
    cut = BRepAlgoAPI_Cut(block, cube)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the block and the cube")
    return cut.Shape()
