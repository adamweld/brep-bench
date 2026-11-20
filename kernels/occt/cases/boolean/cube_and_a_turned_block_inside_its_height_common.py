"""A cube and a turned block lying within its height, their common part. A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 4 turned about the upright and standing from z = 3 to z = 7, one corner at (1, 0, 3), its edges from that corner along (4/5, 3/5, 0), (-3/5, 4/5, 0) and straight up."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    block = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(1, 0, 3), gp.DZ_s(), gp_Dir(4, 3, 0)), 5, 5, 4).Shape()
    common = BRepAlgoAPI_Common(cube, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the block")
    return common.Shape()
