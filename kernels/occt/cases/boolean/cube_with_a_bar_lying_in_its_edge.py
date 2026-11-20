"""A cube united with a bar lying in one of its edges. A cube of side 10 between the origin and (10, 10, 10), united with a bar between the origin and
(10, 3, 3): it runs the cube's whole length along x, hugging the cube's edge along the x axis, and
four of its faces lie in the cube's faces (x = 0, x = 10, y = 0 and z = 0). Everything of the
block lies inside the cube or on its surface, so the union is the cube itself."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 3, 3)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Merge the faces the fuse left split in one plane.
    fuse.SimplifyResult()
    return fuse.Shape()
