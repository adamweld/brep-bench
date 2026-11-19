"""A cube and a block turned with a corner in its face, their common part. The cube from (0, 0, 0)
to (10, 10, 10), and the block 10 by 5 by 10 whose corner is (10, 3, 0), on the cube's face x = 10,
its edges from that corner running 10 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 up."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    # the box's x runs along the placement's x direction, its y along z × x = (−3/5, 4/5, 0)
    block = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(10, 3, 0), gp.DZ_s(), gp_Dir(4, 3, 0)),
                                10, 5, 10).Shape()
    common = BRepAlgoAPI_Common(cube, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the block")
    return common.Shape()
