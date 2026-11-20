"""A cube lengthened by a block with its sides flush. A cube of side 10 between the origin and (10, 10, 10), united with a block between (0, 0, 5) and
(10, 10, 15). The block has the cube's square section and is slid up by 5, so the two overlap in a
block 5 high and all four side faces of one lie in the side planes of the other. The result is one
block 15 high."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 5), gp_Pnt(10, 10, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
