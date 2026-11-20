"""A block with a flat-bottomed blind hole, started above its top face. A block, the box from the origin to (20, 20, 10), with a blind hole of radius 3 in its top face, its axis the vertical line through (10, 10). The hole is 6 deep, from the top face (z = 10) down to a flat floor at z = 4.

If you drill the hole from a start above the top face, make its length reach to the floor at z = 4: the depth is measured from the top face, not from the start."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(20, 20, 10).Shape()
    # the drill starts 2 above the top face and runs 8 down, to the floor at z = 4
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, 10, 12), gp_Dir(0, 0, -1)), 3, 8).Shape()
    cut = BRepAlgoAPI_Cut(block, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the blind hole")
    return cut.Shape()
