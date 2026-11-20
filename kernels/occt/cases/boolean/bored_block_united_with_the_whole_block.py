"""A block with a blind hole united with the same block without it: a block from (0, 0, 0) to
(20, 20, 10), and the same block with a hole of radius 4 bored up from its bottom to z = 6."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    plain = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 20, 10)).Shape()
    hole = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, 10, 0), gp.DZ_s()), 4, 6).Shape()
    bored = BRepAlgoAPI_Cut(plain, hole)
    if not bored.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the block and the hole")
    fuse = BRepAlgoAPI_Fuse(bored.Shape(), plain)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the bored block and the plain block")
    fuse.SimplifyResult()
    return fuse.Shape()
