"""A block with a small cube on one corner of its top: a block from (0, 0, 0) to (20, 15, 6) and a
cube of side 5 from (0, 0, 6) to (5, 5, 11), united."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 15, 6)).Shape()
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 6), gp_Pnt(5, 5, 11)).Shape()
    fuse = BRepAlgoAPI_Fuse(block, cube)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the block and the cube")
    fuse.SimplifyResult()
    return fuse.Shape()
