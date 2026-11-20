"""A cube and a raised turned block poking out of its top, their common part. A cube from (0, 0, 0) to (10, 10, 10), and a block 10 by 5 by 10 turned about the upright and standing from z = 5 to z = 15, one corner at (5, 1, 5), its edges from that corner along (4/5, 3/5, 0), (-3/5, 4/5, 0) and straight up."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    block = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(5, 1, 5), gp.DZ_s(), gp_Dir(4, 3, 0)), 10, 5, 10).Shape()
    common = BRepAlgoAPI_Common(cube, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the block")
    return common.Shape()
