"""A cube united with a block that leaves it past an edge. A cube of side 10 between the origin and (10, 10, 10), united with a block between (5, 0, 0) and
(15, 15, 5): it starts inside the cube, leaves it through the faces x = 10 and y = 10 together,
past their common edge, and lies in the cube's bottom plane z = 0 and its side plane y = 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(5, 0, 0), gp_Pnt(15, 15, 5)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
