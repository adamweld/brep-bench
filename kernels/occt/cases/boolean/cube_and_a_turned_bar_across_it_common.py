"""A cube and a long thin block turned across it, their common part. A cube from (0, 0, 0) to (10, 10, 10), and a block 20 by 1 by 10 turned about the upright. One corner of the block is (0, 1, 0), and the block's three edges from that corner run 20 along (4/5, 3/5, 0), 1 along (−3/5, 4/5, 0) and 10 straight up. Make the solid common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    # the box's x runs along the placement's x direction, its y along z × x = (−3/5, 4/5, 0)
    block = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(0, 1, 0), gp.DZ_s(), gp_Dir(4, 3, 0)),
                                20, 1, 10).Shape()
    common = BRepAlgoAPI_Common(cube, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the block")
    return common.Shape()
