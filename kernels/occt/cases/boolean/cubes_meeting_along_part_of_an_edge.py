"""Two cubes meeting along part of an edge, united. Two cubes of side 10: the first between the origin and (10, 10, 10), the second between (10, 10,
5) and (20, 20, 15), standing diagonally beside it and raised by 5. Their vertical edges at x = y
= 10 lie on one line and overlap in the segment from (10, 10, 5) to (10, 10, 10), which is all
they share. Unite them. The result is the two cubes touching along that segment and nowhere else,
one body only in that sense: its volume and its area are those of the two cubes taken together."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(10, 10, 5), gp_Pnt(20, 20, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
