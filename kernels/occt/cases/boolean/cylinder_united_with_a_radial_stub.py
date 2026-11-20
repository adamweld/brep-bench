"""A cylinder united with a narrower cylinder starting on its axis and leaving through its side. A
cylinder of radius 5 along the z axis from z = −7 to z = 7, and a cylinder of radius 2 along the x
axis from x = 0 to x = 9. Make their union."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    wide = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -7), gp_Dir(0, 0, 1)), 5, 14).Shape()
    stub = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)), 2, 9).Shape()
    fuse = BRepAlgoAPI_Fuse(wide, stub)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two cylinders")
    return fuse.Shape()
