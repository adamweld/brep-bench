"""A cube and a smaller cube meeting at a corner, united. A cube of side 10 between the origin and (10, 10, 10), and a cube of side 4 between (10, 10, 10)
and (14, 14, 14), standing on the first's far corner. They share that one point and nothing else.
Unite them. The result is the two cubes touching at a point, one body only in that sense: its
volume and its area are those of the two cubes taken together."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(10, 10, 10), gp_Pnt(14, 14, 14)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
