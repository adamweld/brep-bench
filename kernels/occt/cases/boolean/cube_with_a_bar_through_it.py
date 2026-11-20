"""A cube united with a bar that passes through it. A cube of side 10 between the origin and (10, 10, 10), united with a bar between (−5, 3, 0) and
(15, 6, 10): it enters the cube through the face x = 0 and leaves through x = 10, strictly inside
the faces across, and is as tall as the cube, so its top lies in the cube's top plane and its
bottom in the cube's bottom plane."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(-5, 3, 0), gp_Pnt(15, 6, 10)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
