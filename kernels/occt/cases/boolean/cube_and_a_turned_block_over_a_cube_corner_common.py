"""A cube and a turned block over its corner and touching two more, their common part. A cube from (0, 0, 0) to (10, 10, 10), and a block 12 by 15 by 10 turned about the upright. One corner of the block is (1, -8, 0), and the block's three edges from that corner run 12 along (4/5, 3/5, 0), 15 along (−3/5, 4/5, 0) and 10 straight up. Make the solid common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    # the box's x runs along the placement's x direction, its y along z × x = (−3/5, 4/5, 0)
    block = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(1, -8, 0), gp.DZ_s(), gp_Dir(4, 3, 0)),
                                12, 15, 10).Shape()
    common = BRepAlgoAPI_Common(cube, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the block")
    return common.Shape()
