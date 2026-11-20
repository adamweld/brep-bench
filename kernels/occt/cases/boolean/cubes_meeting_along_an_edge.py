"""Two cubes meeting along an edge, united. Two cubes of side 10: the first between the origin and (10, 10, 10), the second between (10, 10,
0) and (20, 20, 10), standing diagonally beside it. They touch along the single edge from (10, 10,
0) to (10, 10, 10) and nowhere else. Unite them. The result is the two cubes touching along that
line and nowhere else, one body only in that sense: its volume and its area are those of the two
cubes taken together."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(10, 10, 0), gp_Pnt(20, 20, 10)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
