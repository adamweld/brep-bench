"""A cube with a post through its top, flush with two sides. A cube of side 10 between the origin and (10, 10, 10), united with a post between (0, 0, 5) and
(4, 4, 15): it starts inside the cube, leaves through the top face and stands 5 above it. Two of
its side faces lie in the cube's side faces (x = 0 and y = 0)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 5), gp_Pnt(4, 4, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
