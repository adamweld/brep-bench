"""A cube united with a slab through its middle. A cube of side 10 between the origin and (10, 10, 10), united with a slab between (0, 0, 3) and
(10, 10, 6): it spans the cube both ways in x and y, so its four side faces lie in the cube's side
faces, and it lies strictly between the cube's top and bottom. Everything of the block lies inside
the cube or on its surface, so the union is the cube itself."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 3), gp_Pnt(10, 10, 6)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
