"""A cube united with a pillar standing inside it. A cube of side 10 between the origin and (10, 10, 10), united with a square pillar between (3, 3,
0) and (6, 6, 10): strictly inside the cube across, and as tall as the cube, so its top lies in
the cube's top face and its bottom in the cube's bottom face. Everything of the block lies inside
the cube or on its surface, so the union is the cube itself."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(3, 3, 0), gp_Pnt(6, 6, 10)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
